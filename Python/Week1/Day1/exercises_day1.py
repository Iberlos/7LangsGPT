#Exercise 1
character_name = "Iberlos"
character_level : int = 1
character_experience : float = 0.0

print(f"Character {character_name} is level {character_level} and has {character_experience} expereince points.")
#Quessssstion: can fstrings be broken into multiple lines?
#Exercise 2
temperature_c : float = 10
temperature_f : float = temperature_c * (9/5) +32

print(
    temperature_c,
    " degrees in Celsius is ",
    temperature_f,
    " degrees in Farenhint. " \
    "But Farenhint sucks, so I don't know why you asked."
    )

#Exercise 3
house_alarm = False
power_usage_w = 100.0
home_server_ip = [192,168,0,1]

print(
    "Welcome. Connected to ",
    home_server_ip,
    ".The House alarm is curently ",
    "Armed" if house_alarm else "Disarmed",
    "Power usage since last connection: ",
    power_usage_w
    )

#Exercise 4
base_damage = 10
strength_modifier = 3
critical_multiplier = 1.5

print("The final damage was ", base_damage * strength_modifier * critical_multiplier)

#Exercise 5
player_name = "Iberlos"
level = 10
current_hp = 80
max_hp = 100
xp = 154
armor_rating = 17
is_in_combat = False

print(
    "The character ",
    player_name,
    " has won the battle Thanks to his Armor Rating of ",
    armor_rating,   
    "! He lost ",
    max_hp - current_hp,
    " health points and is at ",
    current_hp/max_hp*100,
    "% of his total health.\n",
    "Total Experience: ",
    xp,
    "\n",
    "This character needs healing!\n" if current_hp<max_hp else "The character didn't take a scratch!\n",
    "Lives remaining 1"
)
#Question: Why end in a newline?
