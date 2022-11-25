use std::io::{Write, Read, Stdin, Stdout};

#[salsa::jar(db = Db)]
pub struct Jar(

);

pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}

#[derive(Default)]
#[salsa::db(crate::Jar)]
pub struct Database {
    storage: salsa::Storage<Self>,
    logs: ()
}

impl salsa::Database for Database {
    fn salsa_event(&self, event: salsa::Event) {
        unimplemented!()
    }
}

#[derive(Debug)]
enum ReadError {
    PrematureEnd,
    UnexpectedChar(char),
    UnexpectedByte(u8),
}

#[derive(Debug)]
enum EvalError {
    EnvLookupError,
}

#[derive(Debug)]
enum PrintError {
    OutputExhausted,
}

#[derive(Debug)]
enum TrucError {
    Read(ReadError),
    Eval(EvalError),
    Print(PrintError),
}

impl From<ReadError> for TrucError {
    fn from(err: ReadError) -> Self { Self::Read(err) }
}

impl From<EvalError> for TrucError {
    fn from(err: EvalError) -> Self { Self::Eval(err) }
}

impl From<PrintError> for TrucError {
    fn from(err: PrintError) -> Self { Self::Print(err) }
}

#[derive(Copy, Clone)]
struct IoArena<'a> {
    stdin: &'a Stdin,
    stdout: &'a Stdout,
}
impl<'a> IoArena<'a> {
    fn stdin(self) -> &'a Stdin { self.stdin }
    fn stdout(self) -> &'a Stdout { self.stdout }
}

trait Truc {
    fn input_port<'a>(&self, db: &dyn Db, arena: IoArena<'a>) -> InputChannel<'a> {
        InputChannel(arena.stdin())
    }
    fn output_port<'a>(&self, db: &dyn Db, arena: IoArena<'a>) -> OutputChannel<'a> {
        OutputChannel(arena.stdout())
    }
    fn read(&self, db: &dyn Db, c: InputChannel<'_>)
            -> Result<Sexp, ReadError>;
    fn eval(&self, db: &dyn Db, input: Sexp)
            -> Result<Value, EvalError>;
    fn print(&self, db: &dyn Db, value: Value, c: OutputChannel<'_>)
             -> Result<Response, PrintError>;
}

struct LeTruc;
impl Truc for LeTruc {
    fn read(&self, db: &dyn Db, c: InputChannel<'_>) -> Result<Sexp, ReadError> {
        
        unimplemented!()
    }
    fn eval(&self, db: &dyn Db, input: Sexp)
            -> Result<Value, EvalError> {
        unimplemented!()
    }
    fn print(&self, db: &dyn Db, value: Value, c: OutputChannel<'_>)
             -> Result<Response, PrintError> {
        unimplemented!()
    }
}

fn main() -> Result<(), TrucError> {
    let mut db = Database::default();

    println!("Hello, world!");

    let le_truc = LeTruc;

    loop {
        let io_arena = IoArena { stdin: &std::io::stdin(), stdout: &std::io::stdout() };
        let input_channel = le_truc.input_port(&db, io_arena);
        let input_sexp = le_truc.read(&db, input_channel)?;
        let value = le_truc.eval(&mut db, input_sexp)?;
        let output_channel = le_truc.output_port(&db, io_arena);
        let resp = le_truc.print(&db, value, output_channel)?;
        if resp.is_goodbye() { break; }
    }

    println!("Goodbye, world!");

    Ok(())
}

struct InputChannel<'a>(&'a mut dyn Read);
struct OutputChannel<'a>(&'a mut dyn Write);
enum Sexp {
    Token(String), // to be replaced by symbols
}
struct Value;
struct Response;

impl<'a> InputChannel<'a> {
    fn read(&mut self) -> Result<Sexp, ReadError> {
        
    }
}

impl Response {
    fn is_goodbye(&self) -> bool { false }
}
