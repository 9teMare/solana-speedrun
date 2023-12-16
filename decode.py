import binascii
import base58

numbers = [175,50,212,96,134,255,44,20,198,240,134,204,94,50,153,1,49,226,120,213,79,107,129,164,231,87,151,191,182,173,165,180,52,134,184,40,184,107,2,164,114,39,241,66,76,153,158,16,142,35,65,243,191,109,119,195,169,121,130,204,25,126,52,23]
first_32 = numbers[:32]
last_32 = numbers[32:]
hex_string = binascii.hexlify(bytearray(first_32)).decode()
base58_string = base58.b58encode(binascii.unhexlify(hex_string)).decode()

hex_last_string = binascii.hexlify(bytearray(last_32)).decode()
base_last_string = base58.b58encode(binascii.unhexlify(hex_last_string)).decode()

print("Private key: ", base58_string)
print("Public key: ", base_last_string)