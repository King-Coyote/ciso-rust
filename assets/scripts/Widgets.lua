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

Widgets.Button = function(_size, _position, _string, _id, _style)

    local _style = _style or defaultStyle

    local button = {
        type = 'button',
        id = _id,
        style = _style,
        properties = {
            position = _position,
            size = _size,
            handleOnClick = function(self, x, y, mouseButton)
                print('durrrr')
            end
        },
    }

    local text = {
        type = 'text',
        id = _id .. '_text',
        style = textStyle,
        font = 'DejaVuSans.ttf',
        properties = {
            string = _string,
            fontSize = 16,
            position = _position,
        }
    }

    Gui:render{
        button,
        text
    }

end

return Widgets