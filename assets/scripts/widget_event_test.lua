

local panel = Gui:add_widget{
    type = 'PANEL',
    properties = {
        size = {100, 100},
        position = {30, 30},
        event_handlers = {
            onClick = function(self, x, y, button)
                print(tostring(self.size[1]))
                print(tostring(x) .. ', ' .. tostring(y))
                print(tostring(button))
            end,
        },
    },
}

panel:set_properties{
    position = {0,0}
}