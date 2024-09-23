local dap = require("dap")
dap.adapters.gdb = {
  type = "executable",
  command = "/usr/bin/rust-gdb",
  args = { "--interpreter=dap", "--eval-command", "set print pretty on" }
}
dap.adapters.lldb = {
    type = "executable",
    command = "/usr/bin/lldb-dap",
    name = "lldb"
}

dap.configurations.rust = {
    {
        name = "Launch",
        type = "gdb",
        request = "launch",
        program = function()
            return vim.fn.getcwd() .. "/target/debug/main"
        end,
        cwd = "${workspaceFolder}",
        stopOnEntry = false,
        args = {
            "-i", "/home/lessu/Downloads/AUTOSAR_00046.xsd"
        }
    }
}

