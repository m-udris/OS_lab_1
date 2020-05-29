//use super::process::Process;
use super::resource_list::ResourceList;
use super::process::Process;
use crate::consts::*;
use crate::real_machine::processor::RMProcessor;
use super::start_stop::StartStop;

pub struct Kernel {
    process_list: Vec<Box<dyn Process>>,
    resources: ResourceList,

}

impl Kernel {
    pub fn new() -> Kernel {
        Kernel {
            process_list: Vec::new(),
            resources: ResourceList::new(),
        }
    }
}

impl Kernel {
    pub fn planner(&mut self, rm:&mut RMProcessor) {
        self.process_list.sort_by_key(|d| d.get_priority());
        let mut queued_processes: Vec<Box<dyn Process>> = Vec::new();
        let mut to_be_removed: Vec<usize> = Vec::new();
        let mut statuses: Vec<(usize, usize)> = Vec::new();
        self.process_list.push(Box::new(StartStop::new(PID_STARTSTOP, 0, 0)));
        loop {
            self.process_list.sort_by_key(|d| d.get_priority());
            for process in &mut self.process_list {
                println!("{:#?}", self.resources);
                if to_be_removed.contains(&process.get_id()) {
                    continue;
                }
                for (p_id, status_id) in &statuses {
                    if process.get_id() == *p_id {
                        process.set_state(*status_id);
                        break;
                    }
                }
                if process.get_state() == P_BLOCKED {
                    let result = process.step(rm);
                    if result.0.is_some() && result.3.is_none()  {
                        process.print(rm);
                        let val = result.0.unwrap();
                        let res = self.resources.take(val, process.get_id());
                        println!("~~~~~~~~~~~~{} {} {}~~~~~~~~~~~~~~~~", val, process.get_id(), res.is_some());
                        if res.is_some() {
                            process.add_resource(res.unwrap());
                        }
                    }
                    if result.1.is_some() || result.2.is_some() || result.3.is_some() {
                        panic!();
                    }
                }
                if process.get_state() == P_READY {
                    // process.print(rm);
                    let result = process.step(rm);
                    // if result.0.is_some() || result.1.is_some() || result.2.is_some() || result.3.is_some() {
                    //     process.print(rm);
                    // }
                    if result.0.is_some() && result.3.is_none() {
                        let value = result.0.unwrap();
                        let gotten_resource = self.resources.take(value, process.get_id());
                        println!("~~~~~~~~~~~~{} {} {}~~~~~~~~~~~~~~~~", value, process.get_id(), gotten_resource.is_some());
                        if gotten_resource.is_some() {
                            process.add_resource(gotten_resource.unwrap());
                        }
                    }
                    else if result.1.is_some() {
                        let value = result.1.unwrap();
                        self.resources.add(value);
                        process.step(rm);
                    }
                    else if result.2.is_some() {
                        let value = result.2.unwrap();
                        queued_processes.push(value);
                    }
                    else if result.3.is_some() && result.0.is_none() {
                        let p_id = result.3.unwrap();
                        to_be_removed.push(p_id);
                        println!("{} will be removed", p_id);
                    }
                    else if result.0.is_some() && result.3.is_some() {
                        let p_id = result.3.unwrap();
                        let status_id = result.0.unwrap();
                        statuses.push((p_id, status_id));
                    }
                    else if !(result.0.is_some() || result.1.is_some() || result.2.is_some() || result.3.is_some()) {
                        let new_res = process.step(rm);
                        if new_res.0.is_some() && new_res.3.is_none() {
                            let value = new_res.0.unwrap();
                            let gotten_resource = self.resources.take(value, process.get_id());
                            println!("~~~~~~~~~~~~{} {} {}~~~~~~~~~~~~~~~~", value, process.get_id(), gotten_resource.is_some());
                            if gotten_resource.is_some() {
                                process.add_resource(gotten_resource.unwrap());
                            }
                        }
                        else if new_res.1.is_some() {
                            let value = new_res.1.unwrap();
                            self.resources.add(value);
                            process.step(rm);
                        }
                        else if new_res.2.is_some() {
                            let value = new_res.2.unwrap();
                            queued_processes.push(value);
                        }
                        else if new_res.3.is_some() && new_res.0.is_none() {
                            let p_id = new_res.3.unwrap();
                            to_be_removed.push(p_id);
                            println!("{} will be removed", p_id);
                        }
                        else if new_res.0.is_some() && new_res.3.is_some() {
                            let p_id = new_res.3.unwrap();
                            let status_id = new_res.0.unwrap();
                            statuses.push((p_id, status_id));
                        }
                    }
                }
            }
            // println!("appened: {:#?}", queued_processes);
            self.process_list.append(&mut queued_processes);
            queued_processes.clear();
            self.process_list.retain(|i| !to_be_removed.contains(&i.get_id()) );
            // println!("cleared: {:?}", to_be_removed);

            to_be_removed.clear();
            for (p_id, status_id) in &statuses {
                self.process_list.iter_mut().find(| x | x.get_id() == *p_id).unwrap().set_state(*status_id);
            }
        }
    } 
}