

local panel = Gui:add_widget{
    type = 'PANEL',
    properties = {
        size = {100, 100},
        position = {30, 30},
        event_handlers = {
            onClick = function(self, x, y, button)
                local new_size_x = self.properties.size[1] - 10
                local new_size_y = self.properties.size[2] - 10
                self:set_properties{size = {new_size_x, new_size_y}}
            end,
        },
    },
}

panel:set_properties{
    position = {0,0}
}