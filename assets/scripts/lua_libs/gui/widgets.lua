local defaultStyle = {
    enabled = {
        bg_color = {50,50,50,255},
        fg_color = {255,255,255,255}
    },
    hovered = {
        bg_color = {75,85,100,255},
        fg_color = {255,255,255,255}
    },
    clicked = {
        bg_color = {95, 115, 120,255},
        fg_color = {255,255,255,255}
    },
}

local textStyle = {
    enabled = {
        fg_color = {150,150,150,255}
    },
    hovered = {
        fg_color = {255,255,255,255}
    },
    clicked = {
        fg_color = {255,255,255,255}
    },
}

local Widgets = {}

Widgets.test = function()
    print('Widgets library has been correctly loaded!')
end

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
            size = {30,30},
            transparent = true,
        }
    }

    local panel = {
        type = 'PANEL',
        children = {text},
        properties = {
            style = _style,
            position = _position,
            size = _size,
        },
    }

    return panel

end

return Widgets