function start()
    info("Start")
    player_speed = player:get_speed()
    info("speed: " .. player_speed)
end

function update()
    if input.held_shift() then
        player:set_speed(player_speed * 2)
    else
        player:set_speed(player_speed)
    end
end