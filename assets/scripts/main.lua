function start()
    info("Start!")
end

function update()
    if key_pressed("A") then
        info("A pressed")
    end
    if key_released("A") then
        info("A released")
    end
end