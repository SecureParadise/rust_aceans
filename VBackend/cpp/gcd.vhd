library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;

entity gcd is
    port(
        a: in integer;
        b: in integer;
        gcd_out: out integer;
        clk: in STD_LOGIC;
        rst: in STD_LOGIC;
        start: in STD_LOGIC;
        done: out STD_LOGIC
    );
end gcd;

architecture fsm of gcd is
    type state_type is (IDLE, INIT, COMPARE, SWAP, MODULO, DONE_STATE);
    signal state, next_state : state_type;
    signal reg_a, reg_b, next_a, next_b : integer;
begin
    -- State register
    process(clk, rst)
    begin
        if rst = '1' then
            state <= IDLE;
            reg_a <= 0;
            reg_b <= 0;
        elsif rising_edge(clk) then
            state <= next_state;
            reg_a <= next_a;
            reg_b <= next_b;
        end if;
    end process;

    -- Next state logic and data path
    process(state, reg_a, reg_b, a, b, start)
    begin
        -- Default values to avoid latches
        next_state <= state;
        next_a <= reg_a;
        next_b <= reg_b;
        done <= '0';
        gcd_out <= 0;

        case state is
            when IDLE =>
                if start = '1' then
                    next_state <= INIT;
                end if;
                
            when INIT =>
                next_a <= a;
                next_b <= b;
                next_state <= COMPARE;
                
            when COMPARE =>
                if reg_b = 0 then
                    next_state <= DONE_STATE;
                elsif reg_b > reg_a then
                    next_state <= SWAP;
                else
                    next_state <= MODULO;
                end if;
                
            when SWAP =>
                next_a <= reg_b;
                next_b <= reg_a;
                next_state <= MODULO;
                
            when MODULO =>
                next_a <= reg_b;
                next_b <= reg_a mod reg_b;
                next_state <= COMPARE;
                
            when DONE_STATE =>
                gcd_out <= reg_a;
                done <= '1';
                next_state <= IDLE;
        end case;
    end process;
end fsm;