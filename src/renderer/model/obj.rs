use crate::renderer::model::Model;
use crate::renderer::gpu::Attribute;
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::path::Path;

pub fn load(path: &Path) -> io::Result<Model> {
    let obj_file = File::open(path)?;
    let obj_file = BufReader::new(obj_file);

    let mut vertices = Vec::<gl::types::GLfloat>::new();
    let mut indices = Vec::<u32>::new();
    let mut vert_cnt = 0;
    for line in obj_file.lines() {
        match line {
            Ok(line) => {
                let mut v = false;
                let mut f = false;
                let mut w = false;
                for (pos, component) in line.split(' ').enumerate() {
                    if pos == 0 {
                        if component == "v" {
                            v = true;
                            f = false;
                        } else if component == "f" {
                            v = false;
                            f = true;
                        } else {
                            break;
                        }
                    } else {
                        if v {
                            if pos == 4 {
                                w = true;
                            }
                            vertices.push(
                                component.parse::<f32>().expect("Could not parse float")
                            );
                            vert_cnt += 1;
                        } else if f {
                            let v_idx = component
                                .split('/')
                                .collect::<Vec<&str>>()
                                .get(0).expect("Could not split on /")
                                .parse::<i32>().expect("Could not parse i32");
                            if v_idx == 0 {
                               panic!("Did not expect 0 in face specification"); 
                            } else if v_idx < 0 {
                                indices.push((vert_cnt - v_idx) as u32);
                            } else {
                                indices.push((v_idx - 1) as u32);
                            }
                        }
                    }
                    
                }
            },
            _ => continue,
        }
    }
    let layout: Vec<Attribute> = vec![Attribute {
        width: 3,
        stride: 3 * std::mem::size_of::<gl::types::GLfloat>(),
        start_idx: 0,
        ty: gl::FLOAT,
    }];

    trace!("DATA: {:?}\n INDICES: {:?}", vertices, indices);

    Ok(
        Model::from_data_and_layout(&vertices, &indices, &layout)
    )
}
