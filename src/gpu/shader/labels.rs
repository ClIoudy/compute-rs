impl super::Shader {
    pub fn object_label(&self) -> String {
        let shader_label = if let Some(label) = &self.shader_label {
            label
        } else {
            "None"
        };

        format!("[Shader label: '{}']", shader_label)
    }
    
    pub fn read_buffers_label(&self, binding: &u32) -> String {
        let x = format!("Read back buffer [binding: '{binding}']");
        let y = self.object_label();
        y + &x
    }
}



/*

pub struct Labels {
    pub shader_label: Option<String>,
    pub shader_group: u32,
    pub verbose: bool,
    // It's very stupid but this is used so that Option<String> can be returned.
    // Otherwise Option<String> would be nececssary, creating a need for 
    puf: String,
}

impl Labels {
    pub fn read_buffers(&mut self, binding: &u32) -> Option<String> {
        let label = format!("Read back buffer [binding: '{binding}']");

        if self.verbose {
            Some(self.normal_label(&label))
        } else {
            Some(self.verbose_label(&label))
        }
    }

    pub fn new(shader_group: u32) -> Self {
        Self {
            shader_label: None,
            shader_group,
            verbose: false,
            puf: String::new()
        }
    }



    pub fn compute_pipeline(&mut self) -> Option<String> {
        let label = "Compute Pipeline";

        if self.verbose {
            Some(self.normal_label(label))
        } else {
            Some(self.verbose_label(label))
        }
    }

    pub fn pipeline_layout(&mut self) -> Option<String> {
        let label = "Pipeline Layout";

        if self.verbose {
            Some(self.normal_label(label))
        } else {
            Some(self.verbose_label(label))
        }
    }
    
    pub fn bind_layout(&mut self) -> Option<String> {
        let label = "Bind Group Layout";

        if self.verbose {
            Some(self.normal_label(label))
        } else {
            Some(self.verbose_label(label))
        }
    }

    pub fn bind_group(&mut self) -> Option<String> {
        let label = "Bind Group";

        if self.verbose {
            Some(self.normal_label(label))
        } else {
            Some(self.verbose_label(label))
        }
    }

    pub fn cpass(&mut self) -> Option<String> {
        let label = "CPASS";

        if self.verbose {
            Some(self.normal_label(label))
        } else {
            Some(self.verbose_label(label))
        }
    }

    pub fn encoder(&mut self) -> Option<String> {
        let label = "Encoder";

        if self.verbose {
            Some(self.normal_label(label))
        } else {
            Some(self.verbose_label(label))
        }
    }

    fn normal_label(&mut self, label: &str) -> String {
        String::new()
    }

    fn verbose_label(&mut self, label: &str) -> String {
        let shader_label = if let Some(x) = &mut self.shader_label {
            x
        } else {
            "None"
        };

        format!("{label}: Shader: [label: '{}', group: {}']", shader_label, self.shader_group)
    }
}


// macro_rules! normal_label {
//     (label:$expr, shader_label:$expr, shader_group:$expr) => {
//         let shader_label = if let Some(x) = &mut self.shader_label {
//             x
//         } else {
//             "None"
//         };

//         format!("{label}, Shader: [label: '{}', group: {}']", shader_label, self.shader_group)
//     };
// }

*/