local defaultStyle = {
    enabled = {
        bgColor = {50,50,50,255},
        fgColor = {255,255,255,255}
    },
    hover = {
        bgColor = {75,85,100,255},
        fgColor = {255,255,255,255}
    },
    clicked = {
        bgColor = {95, 115, 120,255},
        fgColor = {255,255,255,255}
    },
}

local textStyle = {
    enabled = {
        fgColor = {150,150,150,255}
    },
    hover = {
        fgColor = {255,255,255,255}
    },
    clicked = {
        fgColor = {255,255,255,255}
    },
}

local Widgets = {}

Widgets.Button = function(_size, _position, _string, _style)

    local _style = _style or defaultStyle

    -- will eventually be text
    local text = {
        type = 'PANEL',
        properties = {
            style = textStyle,
            string = _string,
            fontSize = 16,
            position = {0,0},
            size = {30,30}
        }
    }

    local panel = {
        type = 'PANEL',
        children = {text},
        properties = {
            style = _style,
            position = _position,
            size = _size,
            event_handlers = {
                onClick = function(self, button, x, y)

                end,
            },
        },
    }

    Gui:add_widget(panel)

end

return Widgets