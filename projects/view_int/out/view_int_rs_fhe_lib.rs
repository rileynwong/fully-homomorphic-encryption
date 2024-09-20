
use rayon::prelude::*;
use std::collections::HashMap;

use phantom_zone::*;

type Ciphertext = FheBool;

enum GateInput {
    Arg(usize, usize), // arg + index
    Output(usize), // reuse of output wire
    Tv(usize),  // temp value
    Cst(bool),  // constant
}

use GateInput::*;

#[derive(PartialEq, Eq, Hash)]
enum CellType {
    AND2,
    NAND2,
    XOR2,
    XNOR2,
    OR2,
    NOR2,
    INV,
    // TODO: Add back MUX2
}

use CellType::*;



fn prune(temp_nodes: &mut HashMap<usize, Ciphertext>, temp_node_ids: &[usize]) {
  for x in temp_node_ids {
    temp_nodes.remove(&x);
  }
}

pub fn view_int(state: &Vec<Ciphertext>) -> Vec<Ciphertext> {
    let parameter_set = get_active_parameter_set();
    rayon::ThreadPoolBuilder::new()
        .build_scoped(
            |thread| {
                set_parameter_set(parameter_set);
                thread.run()
            },
            |pool| pool.install(|| {

                let args: &[&Vec<Ciphertext>] = &[state];

                let mut temp_nodes = HashMap::new();
                let mut out = Vec::new();
                out.resize(32, None);

                let mut run_level = |
                temp_nodes: &mut HashMap<usize, Ciphertext>,
                tasks: &[((usize, bool, CellType), &[GateInput])]
                | {
                    let updates = tasks
                        .into_par_iter()
                        .map(|(k, task_args)| {
                            let (id, is_output, celltype) = k;
                            let task_args = task_args.into_iter()
                            .map(|arg| match arg {
                                Cst(false) => todo!(),
                                Cst(true) => todo!(),
                                Arg(pos, ndx) => &args[*pos][*ndx],
                                Tv(ndx) => &temp_nodes[ndx],
                                Output(ndx) => &out[*ndx]
                                            .as_ref()
                                            .expect(&format!("Output node {ndx} not found")),
                            }).collect::<Vec<_>>();

                            let gate_func = |args: &[&Ciphertext]| match celltype {
                                AND2 => args[0] & args[1],
                                NAND2 => args[0].nand(args[1]),
                                OR2 => args[0] | args[1],
                                NOR2 => args[0].nor(args[1]),
                                XOR2 => args[0] ^ args[1],
                                XNOR2 => args[0].xnor(args[1]),
                                INV => !args[0],
                            };
                            
                            ((*id, *is_output), gate_func(&task_args))
                        })
                        .collect::<Vec<_>>();
                    updates.into_iter().for_each(|(k, v)| {
                        let (index, is_output) = k;
                        if is_output {
                            out[index] = Some(v);
                        } else {
                            temp_nodes.insert(index, v);
                        }
                    });
                };

            

                out[0] = state[0].clone();
    out[10] = state[10].clone();
    out[11] = state[11].clone();
    out[12] = state[12].clone();
    out[13] = state[13].clone();
    out[14] = state[14].clone();
    out[15] = state[15].clone();
    out[16] = state[16].clone();
    out[17] = state[17].clone();
    out[18] = state[18].clone();
    out[19] = state[19].clone();
    out[1] = state[1].clone();
    out[20] = state[20].clone();
    out[21] = state[21].clone();
    out[22] = state[22].clone();
    out[23] = state[23].clone();
    out[24] = state[24].clone();
    out[25] = state[25].clone();
    out[26] = state[26].clone();
    out[27] = state[27].clone();
    out[28] = state[28].clone();
    out[29] = state[29].clone();
    out[2] = state[2].clone();
    out[30] = state[30].clone();
    out[31] = state[31].clone();
    out[3] = state[3].clone();
    out[4] = state[4].clone();
    out[5] = state[5].clone();
    out[6] = state[6].clone();
    out[7] = state[7].clone();
    out[8] = state[8].clone();
    out[9] = state[9].clone();

                out.into_iter().map(|c| c.unwrap()).collect()
            }),
        )
        .unwrap()
}

