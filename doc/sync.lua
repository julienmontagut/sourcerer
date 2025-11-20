-- Conviniency method for settings
local function settings(fileName)
    return dofile(fileName..".settings")
end


-- Load configuration
local workspaces = settings("sync");

for _, value in pairs(workspaces) do
    print("In workspace: ", value)

end
