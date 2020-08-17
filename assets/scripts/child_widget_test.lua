

local Widgets = require "widgets"

local button = Widgets.Button({100, 50}, {50, 50}, "test button")

button.properties.event_handlers = {
    onClick = function(self, button, x, y)
        local pos = self.properties.position
        self:set_properties{position = {pos[1] + 20, pos[2] + 20}}
    end
}

Gui:add_widget(button)