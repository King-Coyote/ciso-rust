local staticStyle = {
    enabled = {
        bgColor = {50,50,50,255},
        fgColor = {255,255,255,255}
    }
}

local textStyleDynamic = {
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

local textStyleStatic = {
    disabled  ={
        fgColor = {255, 255, 255, 255}
    }
}



-- local Widgets = require "Widgets"
-- local startSize = {100, 50}
-- local pos = {Gui:screenWidth() - 100, 0}

-- local makeDynamicText = function(_pos, _string, _onClick)

--     local text =  {
--         type = 'text',
--         font = 'DejaVuSans.ttf',
--         style = textStyleDynamic,
--         properties = {
--             string = _string,
--             position = _pos,
--             fontSize = 24,
--             handleOnClick = _onClick,
--         }
--     }

--     Gui:render{
--         text
--     }

-- end

-- local mainText =  {
--     type = 'text',
--     font = 'DejaVuSans.ttf',
--     style = textStyleStatic,
--     properties = {
--         string = 'A C A D E M Y',
--         position = {10, 20},
--         fontSize = 32,
--     }
-- }

-- Gui:render{
--     mainText
-- }

-- local createTestArea = function()

--     local area = Game:createEntity{
--         space = {
--             level = 100
--         }
--     }

--     print('area created with id ' .. area)

-- end

-- makeDynamicText({10, 200}, 'Create test area', createTestArea)
-- makeDynamicText({10, 250}, 'Exit game', function() Game:exit() end)

-- Widgets.Button(startSize, pos, 'Fuck', 'buttonBasic')

-- local startSize = {100, 50}
-- local pos = {Gui:screenWidth() - 100, 0}

-- local objectSelectorDropdown = {
--     type = "button",
--     id = "objSelectorDropdown",
--     style = staticStyle,
--     properties = {
--         position = {pos[1], pos[2] + startSize[2]},
--         size = {75,100},
--         hidden = true,
--         handleOnNotClick = function(self, x, y, mouseButton)
--             self.call:setProperties{
--                 hidden = true
--             }
--         end,
--     },
-- }

-- local objectSelectorOther = {
--     type = "button",
--     style = dynamicStyle,
--     id = 'objectSelectorOther',
--     properties = {
--         position = {pos[1], pos[2]},
--         size = startSize,
--         handleOnClick = function(self, x, y, mouseButton)
--             objectSelectorDropdown.call:setProperties{
--                 hidden = false,
--             }
--         end
--     },
-- }

-- Gui:render{
--     objectSelectorOther,
--     objectSelectorDropdown
-- }

-- print(objectSelectorDropdown.call)
-- print(objectSelectorOther.call)