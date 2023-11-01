use x86_64::instructions::port::Port;

const PIT_COMMAND_PORT: u16 = 0x43;
const PIT_CHANNEL_2_PORT: u16 = 0x42;
const PC_SPEAKER_CONTROL_PORT: u16 = 0x61;

/// Play a sound of a given frequency for a certain duration.
pub fn play_sound(frequency: u16) {
    // Calculate the PIT value
    let pit_value = 1193180 / u32::from(frequency);

    unsafe {
        // Set PIT to mode 2 (rate generator) for channel 2
        let mut command_port = Port::<u8>::new(PIT_COMMAND_PORT);
        command_port.write(0b10110110);

        // Set PIT frequency for channel 2
        let mut channel_2_port = Port::<u8>::new(PIT_CHANNEL_2_PORT);
        channel_2_port.write((pit_value & 0xFF) as u8); // Low byte
        channel_2_port.write((pit_value >> 8) as u8); // High byte

        // Enable speaker
        let mut speaker_control_port = Port::<u8>::new(PC_SPEAKER_CONTROL_PORT);
        let speaker_state = speaker_control_port.read() | 0b11;
        speaker_control_port.write(speaker_state);
    }

    // Sound is now playing, implement a mechanism to control the duration
}

/// Stop any currently playing sound.
pub fn stop_sound() {
    unsafe {
        // Disable speaker
        let mut speaker_control_port = Port::<u8>::new(PC_SPEAKER_CONTROL_PORT);
        let speaker_state = speaker_control_port.read() & !0b11;
        speaker_control_port.write(speaker_state);
    }
}
