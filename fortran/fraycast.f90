function raycast(fov, cell, map_w, map_h, map, player_x, player_y, player_ang, ray_x, ray_y, draw) result(dist)   
    use iso_c_binding, only: c_float, c_int

    integer(kind=c_int), intent(out) ::  draw
    integer(kind=c_int)              ::  r, fov, dof
    integer(kind=c_int)              ::  cell, map_w, map_h
    integer(kind=c_int)              ::  map_x, map_y, map_pos
    integer(kind=c_int)              ::  map(64)

    real(kind=c_float), parameter    ::  DR = 0.0174532925
    real(kind=c_float), parameter    ::  PI = 3.1415926535
    real(kind=c_float), intent(out)  ::  ray_x, ray_y
    
    real(kind=c_float)   ::  player_x, player_y, player_ang
    real(kind=c_float)   ::  ray_ang  
    real(kind=c_float)   ::  dist_h, dist_v
    real(kind=c_float)   ::  hor_x, hor_y, ray_hx, ray_hy
    real(kind=c_float)   ::  vert_x, vert_y, ray_vx, ray_vy
    real(kind=c_float)   ::  offset_x, offset_y

    real(kind=c_float)   :: a_tan, n_tan

    
    ray_ang = player_ang
    if(ray_ang < 0) then 
        ray_ang = ray_ang + 2*PI
    end if
    if(ray_ang > 2*PI) then
        ray_ang = ray_ang - 2*PI
    end if

    print *, "PA [F]: ", player_ang

    do r = 1, 1
    draw = 1
    ! Horizontal line
        dist_h = 1000000
        hor_x = player_x
        hor_y = player_y
        

        dof = 0
        a_tan = -1/atan(ray_ang)

        if(ray_ang > PI) then
            ray_hy = ((int(player_y)/cell)*cell) - 0.0001
            ray_hx = (player_y-ray_hy) * a_tan + player_x
        
            offset_y = -cell
            offset_x = -offset_y * a_tan
        end if
        
        if(ray_ang < PI) then
            ray_hy = ((int(player_y)/cell)*cell) + cell
            ray_hx = (player_y-ray_hy) * a_tan + player_x

            offset_y = cell
            offset_x = -offset_y * a_tan
        end if

        if((ray_ang == 0) .or. (ray_ang == PI)) then
            ray_hx = player_x
            ray_hy = player_y
            dof = 8
        end if


        do while(dof < 8) 
            map_x = int(ray_hx)/cell
            map_y = int(ray_hy)/cell
            map_pos = map_y * map_w + map_x

            if((map_pos > 0) .and. (map_pos < (map_w * map_h)) .and. (map(map_pos) > 0)) then
                hor_x = ray_hx
                hor_y = ray_hy
                ! dist_h = fdist(player_x, player_y, hor_x, hor_y)
                dist_h = sqrt((vert_x-player_x)**2 + (vert_y-player_y)**2)
                print *, "DIST_H [F]: ", dist_h
                print *, "RX(H): [F]:", ray_hx
                print *, "RY(H): [F]:", ray_hy

                dof = 8
            else 
                ray_hx = ray_hx + offset_x
                ray_hy = ray_hy + offset_y
                dof = dof + 1
            end if
        end do


    ! Vertical line
        dist_v = 1000000
        vert_x = player_x
        vert_y = player_y
        

        dof = 0
        n_tan = -tan(ray_ang)

        if((ray_ang > PI_2) .and. (ray_ang < 3*PI_2)) then
            ray_vx = ((int(player_x)/cell)*cell) - 0.0001
            ray_vy = (player_x-ray_vx) * n_tan + player_y

            offset_x = -cell
            offset_y = -offset_x * n_tan
        end if
        
        if((ray_ang < PI_2) .and. (ray_ang > 3*PI_2)) then
            ray_vx = ((int(player_x)/cell)*cell) + cell
            ray_vy = (player_x-ray_vx) * n_tan + player_y

            offset_x = cell
            offset_y = -offset_x * n_tan
        end if

        if((ray_ang == 0) .or. (ray_ang == PI)) then
            ray_vx = player_x
            ray_vy = player_y
            dof = 8
        end if


        do while(dof < 8) 
            map_x = int(ray_vx)/cell
            map_y = int(ray_vy)/cell
            map_pos = map_y * map_w + map_x

            if((map_pos > 0) .and. (map_pos < (map_w * map_h)) .and. (map(map_pos) > 0)) then
                vert_x = ray_vx
                vert_y = ray_vy
                ! dist_v = fdist(player_x, player_y, vert_x, vert_y)
                dist_v = sqrt((vert_x-player_x)**2 + (vert_y-player_y)**2)
                print *, "DIST_V [F]:", dist_v
                print *, "RX(V): [F]:", ray_vx
                print *, "RY(V): [F]:", ray_vy

                dof = 8
            else 
                ray_vx = ray_vx + offset_x
                ray_vy = ray_vy + offset_y
                dof = dof + 1
            end if
        end do


        ! Shortest line
        if(dist_v < dist_h) then
            dist  = dist_v
            ray_x = vert_x
            ray_y = vert_y
        end if
        if(dist_h < dist_v) then
            dist  = dist_h
            ray_x = hor_x
            ray_y = hor_y
        end if

        print *, "DIST [F]: ", dist
        print *, "RX: [F]: ", ray_x
        print *, "RY: [F]: ", ray_y
        print *, "RA: [F]: ", ray_ang

        ray_ang = ray_ang + DR
        if(ray_ang < 0) then      
            ray_ang = ray_ang + 2*PI;
        end if
        if(ray_ang > 2*PI) then
            ray_ang = ray_ang - 2*PI;
        endif
    end do
end function raycast