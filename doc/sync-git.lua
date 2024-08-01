-- List files and folders
local function ls(directory)
    local i, t, popen = 0, {}, io.popen
    local pfile = popen('ls -a "'..directory..'"')
    for filename in pfile:lines() do
        i = i + 1
        t[i] = filename
    end
    pfile:close()
    return t
end

-- Finds all repositories on GitHub
local function find_gitlab_repositories()

end

-- Finds all repositories in workspace
local function find_repositories(directory)
    local i, t, popen = 0, {}, io.popen
    local pfile = popen('find "'..directory..'" -name .git -type d')
    for filename in pfile:lines() do
        i = i + 1
        t[i] = filename
    end
    pfile:close()
    return t
end

local function get_repository_root(path)
    local pfile = io.popen('cd "'..path..'/.." && git rev-parse --show-toplevel')
    local t = pfile:read()
    pfile:close()
    return t
end

local function run_command(command)
    local process = io.popen(command)
    local output = process:read()
    process:close()
    return output
end

local repos = find_repositories(".")

print("# Repos with uncommited changes")
for _,repo in ipairs(repos) do
    local repoRoot = get_repository_root(repo)
    local out = run_command('cd "'..repoRoot..'" && git status --short')
    if out ~= nil then
        print(" - "..repoRoot)
    end
end
print("===")

print()
print("# Repos not on master")
print("===")
for _,repo in ipairs(repos) do
    local repoRoot = get_repository_root(repo)
    local out = run_command('cd "'..repoRoot..'" && git branch --show-current')
    if out ~= "master" then
        print(" - "..repoRoot.." ("..out..")")
    end
end
print("===")

print()
print("# Repos without remote")
print("===")
for _,repo in ipairs(repos) do
    local repoRoot = get_repository_root(repo)
    local out = run_command('cd "'..repoRoot..'" && git remote')
    if out == nil then
        print(" - "..repoRoot)
    end
end
