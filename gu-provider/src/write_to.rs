use futures_cpupool::CpuPool;
use bytes::Bytes;
use futures::prelude::*;
use futures::Async;
use std::fs::File;
use std::path::Path;
use futures::future;
use actix::{Actor, Context, Message, Handler, SystemService, Supervised};
use std::io::{self, Seek, SeekFrom, Write};

struct FileWriter {
    pool: CpuPool
}

impl Default for FileWriter {
    fn default() -> FileWriter {
        FileWriter {
            pool: CpuPool::new_num_cpus(),
        }
    }
}

impl Supervised for FileWriter {}

impl SystemService for FileWriter {}

impl Actor for FileWriter {
    type Context = Context<Self>;
}

struct WriteToFile {
    file: File,
    x: Bytes,
    pos: u64,
}

impl Message for WriteToFile {
    type Result = ();
}

impl Handler<WriteToFile> for FileWriter {
    type Result = ();

    fn handle(&mut self, msg: WriteToFile, ctx: &mut Context<Self>) -> () {
        use actix::AsyncContext;
        use actix::WrapFuture;

        ctx.spawn(write_chunk(msg.file, msg.x, msg.pos, self.pool.clone()).map_err(|_| ()).into_actor(self));
    }
}

struct WithPositions<S: Stream<Item=Bytes, Error=()>> {
    stream: S,
    pos: u64,
}

impl<S: Stream<Item=Bytes, Error=()>> WithPositions<S> {
    pub fn new(a: S) -> WithPositions<S> {
        Self {
            stream: a,
            pos: 0,
        }
    }
}

impl<S: Stream<Item=Bytes, Error=()>> Stream for WithPositions<S>
{
    type Item = (Bytes, u64);
    type Error = ();

    fn poll(&mut self) -> Result<Async<Option<(Bytes, u64)>>, ()> {
        match self.stream.poll() {
            Ok(Async::Ready(Some(x))) => {
                let len = x.len() as u64;
                let res = Ok(Async::Ready(Some((x, self.pos))));
                self.pos += len;

                res
            },
            Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

fn write_chunk(mut file: File, x: Bytes, pos: u64, pool: CpuPool) -> impl Future<Item=(), Error=io::Error> {
    pool.spawn_fn(move || {
        future::result(file.seek(SeekFrom::Start(pos)))
            .and_then(move |_| {
                file.write(x.as_ref()).and_then(|_| Ok(()))
            })
    })
}

pub fn write_async<Ins: Stream<Item = Bytes>, P: AsRef<Path>>(
    input_stream: Ins,
    path: P,
) -> impl Future<Item=(), Error=()> {
    future::result(File::create(path).map_err(|_| ()))
        .and_then(|file| {
            WithPositions::new(input_stream.map_err(|_| error!("Input stream error")))
                .for_each(move |(x, pos)| {
                    future::result(file.try_clone()).map_err(|e| error!("File clone error {:?}", e))
                        .and_then(move |file| {
                            let msg = WriteToFile { file, x, pos };
                            FileWriter::from_registry().send(msg).map_err(|e| error!("FileWriter error: {:?}", e))
                        })
                })
        })
}


#[cfg(test)]
mod tests {
    use futures::stream;
    use futures::prelude::*;
    use bytes::Bytes;
    use std::path::PathBuf;
    use write_to::write_async;
    use actix::System;
    use actix::Arbiter;

    #[test]
    #[ignore]
    fn it_works() {
        let stream = stream::iter_ok::<_, ()>(1..300).map(|a| Bytes::from(format!("{:?} ", a)));

        let _ = System::run(|| {
            Arbiter::spawn(write_async(stream, PathBuf::from("abcd")).then(|_| Ok(System::current().stop())))
        });
    }
}
