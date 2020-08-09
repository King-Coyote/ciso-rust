local defaultStyle = {
    enabled = {
        fgColor = {255, 255, 255, 255},
        bgColor = {50, 50, 50, 255},
        outlineThickness = 0.0,
        outlineColor = {255, 255, 255, 255}
    },
    hover = {
        fgColor = {255, 255, 255, 255},
        bgColor = {140, 140, 100, 255},
        outlineColor = {255, 255, 255, 255},
        outlineThickness = 1.0
    },
    clicked = {
        fgColor = {255, 255, 255, 255},
        bgColor = {200, 200, 160, 255},
        outlineColor = {255, 255, 255, 255},
        outlineThickness = 1.0
    }
}

local panelStyle = {
    enabled = {
        bgColor = {0,0,0,0}
    }
}

local defaultStyle2 = {
    enabled = {
        bgColor = {0, 86, 67, 255},
        outlineThickness = 1.0,
        outlineColor = {255, 255, 255, 255}
    }
}

local close = function(self, x, y, button)
    self:getParent():close()
end

local backPanel = Gui:newObject{"panel", {
    id = "MainPanel",
    size = {Gui:screenWidth(), Gui:screenHeight()},
    position = {0,0},
    style = panelStyle,
    handleOnClick = function(self, x, y, button)
        -- if button == 1 then
        --     Gui:focus(tooltip)
        --     tooltip:setProperties{
        --         hidden = false,
        --         position = {x,y}
        --     }
        --     tooltip:setVar("clickPos", {x,y})
        -- end
        if button == 1 then
            local butt = Gui:newObject{"panel", {
                id = "Tooltip",
                size = {100, 120},
                position = {x, y},
                style = defaultStyle2,
                handleOnNotClick = function(self)
                    self:close()
                end,
                children = {
                    {"text", {
                        id="toolText",
                        position={0,0},
                        string="What do?",
                        font = "DejaVuSans.ttf",
                        style= defaultStyle,
                        fontSize = 12
                    }},
                    {"button", {
                        id="toolButt1",
                        position={0,20},
                        size={100,50},
                        style=defaultStyle,
                        handleOnClick = close
                    }},
                    {"button", {
                        id="toolButt2",
                        position={0,70},
                        size={100,50},
                        style=defaultStyle,
                        handleOnClick = close
                    }},
                },
            }}
        end
    end,
}}

-- local title = Gui:newText{
--     id="titleText",
--     position={150, 50},
--     string="A C A D E M Y",
--     font = "DejaVuSans.ttf",
--     style= defaultStyle,
--     fontSize = 72
-- }

-- local buttonStart = Gui:newButton{
--     id = "StartButton",
--     size = {200,100},
--     position = {200, 250},
--     style = defaultStyle2
-- }

-- local buttonOverlay = Gui:newButton{
--     id="shithead",
--     size = {30,30},
--     position = {20,20},
--     style = defaultStyle,
--     parent = buttonStart
-- }

-- buttonStart:addEventListener(
--     GuiEventType["click"],
--     function()
--         print(Gui:screenWidth())
--     end
-- )

-- buttonOverlay:addEventListener(
--     GuiEventType["click"],
--     function()
--         buttonStart:close()
--     end
-- )

-- local field = Gui:newTextField{
--     id="field1",
--     position = {10,10},
--     size={200,40},
--     style = defaultStyle,
--     fontSize = 36
-- }

-- local buttonLabel = Gui:newText{
--     id="startButtonLabel",
--     position={5, 5},
--     string="GENERATE SCENE",
--     font = "DejaVuSans.ttf",
--     style= defaultStyle2,
--     fontSize = 16,
--     parent = buttonStart
-- }