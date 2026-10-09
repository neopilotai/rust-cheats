local M = {}

function M.search(query)
  query = query or vim.fn.input("Cheat-sheet search: ")
  if query == "" then return end
  vim.fn.jobstart({ "rust-cheats", "search", query }, {
    stdout_buffered = true,
    on_stdout = function(_, data)
      if not data then return end
      vim.schedule(function()
        vim.cmd("botright new")
        local buf = vim.api.nvim_get_current_buf()
        vim.bo[buf].buftype = "nofile"
        vim.bo[buf].bufhidden = "wipe"
        vim.bo[buf].swapfile = false
        vim.api.nvim_buf_set_lines(buf, 0, -1, false, data)
        vim.bo[buf].modifiable = false
      end)
    end,
  })
end

vim.api.nvim_create_user_command("RustCheats", function(opts)
  M.search(opts.args)
end, { nargs = "*" })

return M
