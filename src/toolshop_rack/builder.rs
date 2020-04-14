use crate::toolshop_rack::types::ToolType;

pub struct Tool {
  pub name: String,
  pub tool_type: crate::toolshop_rack::types::ToolType,
}

pub struct Toolbox {
  name: String,
  tools: Vec<Tool>,
}

impl Tool {
  pub fn new(name: String, tool_type: ToolType) -> Tool {
    Tool {
      name,
      tool_type,
    }
  }
}

impl Toolbox {
  pub fn new() -> Toolbox {
    Toolbox {
      name: String::from("Steve's Toolbox"),
      tools: vec!
        [
          Tool::new(String::from("Saw"), ToolType::CUT),
          Tool::new(String::from("Hammer"), ToolType::HIT),
          Tool::new(String::from("Scissors"), ToolType::CUT),
        ]
    }
  }

  pub fn add_tool (&mut self, name: &str, tool_type: ToolType) {
    self.tools.push(Tool {
        name: String::from(name),
        tool_type
      }
    );
  }

  pub fn yield_tool(self, name: &str) -> Option<Tool> {
    let tools = self.tools;
    for tool in tools {
      if tool.name == name {
        return Some(tool);
      }
    }
    None
  }
}