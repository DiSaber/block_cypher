# Block Cypher
## Installation 
To install the program, open the latest release from [here](https://github.com/DiSaber/block_cypher/releases/latest) and download the `block_cypher.exe` file. You can place it anywhere but you should consider placing it on your desktop or creating a shortcut for your own convinience. Once you have set your password you can go through the quantum resistant key exchange process with someone to send encrypted messages.
## Usage
### Key Exchange
After following the installation instructions above and creating your password, you should be on the menu screen with several options. To start a key exchange and begin sending encrypted messages, navigate to `Contacts -> Add Contact -> Start Key Exchange`. Once in this menu, you may enter any contact name and click the `Copy Receiving Key` button. Instruct the other user to follow the installation instructions and navigate to `Contacts -> Add Contact -> Enter a Receiving Key`. You may now send the copied receiving key to the other user. They will see a similar menu where they can enter any contact name and paste the receiving key in a text field. Once they've pasted the receiving key, instruct them to click the `Copy Cipher Text` button and send this cipher text back to you. They may now click the `Add Contact` button. On your end, paste the cipher text and click the `Add Contact` button. The key exchange is now complete.
### Verification
In the contacts menu, you can select the other user's name in the dropdown and observe the "view key" to the left. You should compare the view keys (preferably in-person) to ensure they are identical. If they don't match, you should redo the key exchange.
### Encryption/Decryption
You should now be able to encrypt/decrypt messages and files in their respective menus.
## Best practices
I highly suggest you write down or otherwise store your password in a secure location as there is no way to recover it. If you want certain messages you receive to be safe from deletion, you may want to store the encrypted copy somewhere on your device.
## Resetting Password
Unfortunately there is no way to recover your password as AES-256 is irreversible. You can however delete the file titled `block_cypher.data` in the directory `C:\Users\{username}\AppData\Roaming\DiSaber\BlockCypher\config` to reset the application.
## Technical specifications
I use Kyber-1024 for the key exchange and AES-256 for data storage and message sharing.
