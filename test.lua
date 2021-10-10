type mod = {
    Hello: () -> nil,
    TakeArgs: (string, number) -> boolean
}

--[[--

--]]
local mod = {}

function mod.new()
    return setmetatable({}, mod)
end

function mod:Hello()
    print("test")
end

function mod:TakeArgs(test: string, other_test: number): boolean
    print("yoo")
end

return mod