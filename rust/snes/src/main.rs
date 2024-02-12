use memory::Memory;
// use state::State;
use ricoh5a22::Ricoh5A22;

fn main() {
    let bin = vec![0x69, 0xFF, 0xFF, 0x69, 0x01, 0x00];
    let mut cpu = Ricoh5A22::default();
    cpu.load(bin);
    let op = cpu.fetch();
    cpu.execute(&op);
    println!("{}", cpu);
    let op = cpu.fetch();
    cpu.execute(&op);
    println!("{}", cpu);
}
