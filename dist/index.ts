import { tauri } from "@tauri-apps/api"

console.log("テスト")

const main = async() => {
  const input = +(document?.getElementById("input")?.getAttribute("value") || 5)
  const perfects: number[] = await tauri.invoke("perfect", {max: input})
  const output = document.getElementById("output") as HTMLElement
  perfects.forEach(p=>{
    const newEl = document.createElement("li")
    newEl.innerText = `${p}`
    output.appendChild(newEl)
  })
}

window.onload = () => {
  const button = document.getElementById("button") as HTMLElement
  button.innerText = "完全数を計算";
  button.addEventListener("click", main)
}
