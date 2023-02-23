let renderedObjects = [];

import("../pkg/index.js").then(({ add_obj, get_obj }) => {
    document.getElementById("addTrigger").addEventListener("click", event => {
        const name = document.getElementById("name").value;
        const color = document.getElementById("color").value;

        add_obj(color, name);

        // render all children now
        const livingObjects = get_obj();

        // this won't render duplicates
        livingObjects.filter((v) => !renderedObjects.includes(v)).map(str => {
            const _str = str.split("--");
            return {
                name: _str[0],
                color: _str[1]
            }
        }).map(item => {
            const element = document.createElement("div");
            element.style.backgroundColor = item.color;
            const text = document.createElement("p");
            text.innerText = item.name;

            element.appendChild(text);

            const container = document.getElementById("cont");
            container.appendChild(element);
        });
        renderedObjects = livingObjects;
        console.log(get_obj());

    });
})
    .catch(console.error);

