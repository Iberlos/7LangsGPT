//Exercise 1
let gameTitle : string = "MyGame"
let versionNumber : number = 1.0
let isReleased : boolean = true

console.log(
    "Game Title: %s\nVersion Number : %d\nReleased: %s",
    gameTitle,
    versionNumber,
    isReleased ? "Yes" : "No"
)

//Exercise 2
let temperatureC : number = 23.7
let temperatureF : number = (temperatureC * 9/5) + 32

console.log(
    "%d°C = %d°F",
    temperatureC,
    temperatureF
)

//Exercise 3
let isDoorLocked : boolean = false
let currentTime : number = 23

console.log(
    currentTime > 22 || currentTime < 6 ? "Auto-Lock engaged." : "Daytime Mode"
)
//Note: this one got me good. I could swear the logical check should be "currentTime > 22 & currentTime < 6", but I see my error now. Just need to own it...

//Exercise 4
let health : number = 29
let armor : number = 15
let isInCombat : boolean = true

console.log(
    "Character Status:\n%sArmor: %d\nCurrently %s",
    health <= 0 ? "Dead\n" : health <30 ? "Critical\n" : "",
    armor,
    isInCombat ? "In Combat" : "Idle"
)

//Exercise 5
type weaponType = "Sword" | "Spear"
interface weapon {
    type: weaponType,
    name: string,
    damage: number
}
const sword : weapon = {type: "Sword", name: "Butter Knife", damage:1}
const spear : weapon = {type: "Spear",name: "Tooth Pick", damage:3}
//Question: I intended to complete the "Narrowing using typeof" objective by testing for types of weapons. However typeOf weapon === "sword" does not work out, why?
interface buff {
    name: string,
    affects: string,
    value: number,
    message: string
}
type healthStatus = "Dead" | "Critical" | "Wounded" | "Healthy";
const healthBuff : buff = {name: "Divine Health", affects: "maxHealth", value: 30, message: "The divines bless you with vigor!"}
const healthCurse : buff = {name: "Cursed Health", affects: "maxHealth", value: -30, message: "The devil curses you with frailty!"}
const player : {
    name: string,
    level: number,
    xp: number,
    healthStatus: healthStatus,
    health: number,
    maxHealth: number,
    stamina: number,
    buffs: buff[],
    weapon?: weapon
} = {name: "Iberlos", level: 1, xp: 0,healthStatus: "Healthy", maxHealth: 100, health: 50, stamina: 100, buffs: [healthBuff, healthCurse], weapon: sword}
const tempPlayerHealthFraction = player.health/player.maxHealth

let message: string = `
Player ${player.name} is ${player.healthStatus}.
${player.health<player.maxHealth ? `Health is at ${player.health/player.maxHealth*100}%`: ""}
${player.weapon ? `${player.name} is holding a ${player.weapon.type}. Its name is ${player.weapon.name} and it deals ${player.weapon.damage} damage.`:`${player.name} is unarmed.`}
Active Effects:
${player.buffs.map(b => b.message).join("\n")}
`
//Question: "${player.buffs.forEach(element => {element.message})}" The forEach doesn't seem to work properly, but it doesn't cause an error. What is wrong with it?
//Answer: the foreach returns null, you need a function that is able to attach the strings to the message. "${player.buffs.map(b => b.message).join("\n")}" works.
console.log(message)

