use basic_5::entry as entry_basic_5;
use basic_5::ID as PROGRAM_ID_BASIC_5;
const PROGRAM_NAME_BASIC_5: &str =  "basic_5";
use fuzz_instructions::basic_5_fuzz_instructions::FuzzInstruction as FuzzInstruction_basic_5;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_basic_5;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(PROGRAM_NAME_BASIC_5,&PROGRAM_ID_BASIC_5,processor!(convert_entry!(entry_basic_5)));

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_BASIC_5, &mut client);
        });
    }
}
