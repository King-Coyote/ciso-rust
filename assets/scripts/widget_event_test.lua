

local panel = Gui:add_widget{
    type = 'PANEL',
    properties = {
        size = {100, 100},
        position = {30, 30},
        onClick = function()
            print("I've been clicked!")
        end
    }
}

panel:set_properties{
    position = {0,0}
}