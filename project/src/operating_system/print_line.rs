use super::process::Process;
use crate::real_machine::processor::RMProcessor;
use crate::consts::*;
use super::resource::Resource;

pub struct PrintLine {
    id: usize,
    parent_id: usize,
    vm: usize,
    state: usize,
    section: usize,
    resources: Vec<Resource>,
    priority: usize,
}

impl PrintLine {
    pub fn new(id: usize, parent_id: usize, vm: usize) -> PrintLine {
        PrintLine {
            id: id,
            parent_id: parent_id,
            vm: vm,
            state: P_READY,
            section: 0,
            resources: Vec::new(),
            priority: 3
        }
    }
}

impl Process for PrintLine {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_state(&self) -> usize {
        self.state
    }
    fn set_state(&mut self, state: usize) {
        self.state = state;
    }
    fn get_vm(&self) -> usize {
        self.vm
    }
    fn set_vm(&mut self, vm: usize) {
        self.vm = vm;
    }
    fn add_resource(&mut self, res: Resource) {
        self.resources.push(res);
    }
    fn take_resource(&mut self, resource_type: usize) -> Resource {
        let mut resource_index = self.resources.len();
        for (index, res) in self.resources.iter().enumerate() {
            if res.get_type() == resource_type {
                resource_index = index;
                break;
            }
        }
        if resource_index == self.resources.len() {
            panic!();
        }
        self.resources.remove(resource_index)
    }
    fn has_resource(&self, resource_type: usize) -> bool {
        for res in &self.resources {
            if res.get_type() == resource_type {
                return true;
            }
        }
        false
    }
    fn step(&mut self, rm: &mut RMProcessor) -> (Option<usize>, Option<Resource>, Option<Box<dyn Process>>, Option<usize>) {
        match self.section {
            0 => {
                if self.has_resource(RES_LINE_IN_MEM) {
                    self.section += 1;
                    self.state = P_READY;
                    (None, None, None, None)
                }
                else {
                    return (Some(RES_LINE_IN_MEM), None, None, None);
                }
            },
            1 => {
                if self.has_resource(RES_CHNL) {
                    self.section += 1;
                    self.state = P_READY;
                    (None, None, None, None)
                }   
                else {
                    return (Some(RES_CHNL), None, None, None);
                }
                
            },
            2 => {
                let source_id = 0;
                let destination_id = 4;
                self.print_msg(rm);
                self.section += 1;
                (None, None, None, None)
            },
            3 => {
                self.section = 0;
                return(None, Some(self.take_resource(RES_CHNL)), None, None);
            },
            _ => panic!(),
            
        }     
    }
    fn print(&self, rm:& RMProcessor){
        println!("Process: PrintLine {}", self.get_id());
        print!("Status: ");
        match self.state {
            P_READY => println!("P_READY"),
            P_RUNNING => println!("P_RUNNING"),
            P_BLOCKED => println!("P_BLOCKED"),
            P_READY_SUSP => println!("P_READY_SUSP"),
            P_BLOCKED_SUSP => println!("P_BLOCKED_SUSP"),
            _ => println!("INVALID STATE"),
        }
        println!("Section: {}", self.section);
        for resource in &self.resources {
            print!("Resource: ");
            match resource.get_type() {
                RES_S_MEM => println!("RES_S_MEM"),
                RES_U_MEM => println!("RES_U_MEM"),
                RES_DISK => println!("RES_DISK"),
                RES_CHNL => println!("RES_CHNL"),
                RES_TASK_IN_SUPER => println!("RES_TASK_IN_SUPER"),
                RES_FROM_USER_INT => println!("RES_FROM_USER_INT"),
                RES_FILE_PACK => println!("RES_FILE_PACK"),
                RES_USER_INPUT => println!("RES_USER_INPUT"),
                RES_LINE_IN_MEM => println!("RES_LINE_IN_MEM"),
                RES_FROM_FILEWORK => println!("RES_FROM_FILEWORK"),
                RES_INTERRUPT => println!("RES_INTERRUPT"),
                RES_FROM_INTERRUPT => println!("RES_FROM_INTERRUPT"),
                RES_THEAD_SUPER => println!("RES_THEAD_SUPER"),
                RES_TPROG_SUPER => println!("RES_TPROG_SUPER"),
                RES_TASK_IN_USER => println!("RES_TASK_IN_USER"),
                RES_TDAT_SUPER => println!("RES_TDAT_SUPER"),
                _ => println!("INVALID RESOURSE"),
            }
        }
        println!();
    }
    fn get_priority(&self) -> usize {
        self.priority
    } 
}
impl PrintLine{
    fn print_msg(&mut self, rm:&mut RMProcessor) {
        let message = self.take_resource(RES_LINE_IN_MEM).get_msg();
        // for resource in &self.resources {
        //     if resource.get_type() == RES_LINE_IN_MEM {
        //         message = self.resources.remove(resource.get_type()).get_msg().to_owned();
        //         break;
        //     }      
        // }
        let letter = message.chars().next();
        let printing = &message[1..];
        match letter.unwrap() {
            'e' => println!("{}", printing),
            'n' => {
                let vm = printing.parse::<usize>().unwrap();
                rm.get_vars(vm + 10);
                rm.process_prtn();
                rm.set_vars(vm + 10);
            },
            's' => {
                let vm = printing.parse::<usize>().unwrap();
                rm.get_vars(vm + 10);
                rm.process_prts();
                rm.set_vars(vm + 10);
            },
             _ => println!("Invalid type for printing!"),
        }
    }
    
}