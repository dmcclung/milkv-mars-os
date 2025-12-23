# Simple Interactive Serial Port Terminal in PowerShell

# --------------------- Configuration ---------------------
$PortName = "COM4"          # Change to your COM port (e.g., COM1, COM4)
$BaudRate = 115200          # Common values: 9600, 19200, 115200
$Parity   = "None"          # None, Odd, Even, Mark, Space
$DataBits = 8               # Usually 7 or 8
$StopBits = "One"           # One, OnePointFive, Two
# ---------------------------------------------------------

# List available ports for reference (optional)
Write-Host "Available COM ports:"
[System.IO.Ports.SerialPort]::GetPortNames()

try {
    # Create and configure the serial port object
    $port = New-Object System.IO.Ports.SerialPort $PortName, $BaudRate, $Parity, $DataBits, $StopBits
    $port.ReadTimeout = 500     # Timeout for ReadLine (ms) - adjust if needed
    $port.Open()
    Write-Host "`nSerial port $PortName opened successfully at $BaudRate baud."
    Write-Host "Type messages and press Enter to send. Type 'exit' to quit.`n"

    # Main loop: read user input and send, while displaying received data
    while ($true) {
        # Check if there is data available to read
        if ($port.BytesToRead -gt 0) {
            # Read all available data (use ReadExisting for raw output)
            $received = $port.ReadExisting()
            Write-Host -NoNewline $received  # -NoNewline preserves formatting
        }

        # Check for user input (non-blocking)
        if ([Console]::KeyAvailable) {
            $input = [Console]::ReadLine()

            if ($input -eq "exit") {
                break
            }

            if ($input) {
                # Send the input followed by a newline (common for devices)
                $port.WriteLine($input)
                Write-Host "Sent: $input"
            }
        }

        # Small delay to prevent high CPU usage
        Start-Sleep -Milliseconds 100
    }
}
catch {
    Write-Error "Error opening or using serial port: $_"
}
finally {
    if ($port.IsOpen) {
        $port.Close()
        Write-Host "`nSerial port closed."
    }
    $port.Dispose()
}