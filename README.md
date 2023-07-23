# Block Cypher
## Installation 
To install the program just download the latest release from [here](https://github.com/DiSaber/block_cypher/releases/latest). You can place it anywhere but you should consider placing it on your desktop or creating a shortcut for your own convinience. Once you have set your password you can go through the quantum resistant key exchange process with someone to send encrypted messages.
## Best practices
I highly suggest you write down or otherwise store your password in a secure location as there is no way to recover it. If you want certain messages you recieve to be safe from deletion, you may want to store the encrypted copy somewhere on your device.
## Resetting Password
Unfortunately there is no way to recover your password as AES-256 is irreversible. You can however delete the file titled `block_cypher.dat` in the directory `C:\Users\{username}\AppData\Roaming\DiSaber\BlockCypher\config` to reset the application.
## Technical specifications
I use Kyber-1024 for the key exchange and AES-256 for data storage and message sharing.
