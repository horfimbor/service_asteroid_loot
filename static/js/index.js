class AsteroidLoot extends HTMLElement {

    constructor() {
        super();

        this.attachShadow({ mode: 'open' });
        this._render()
    }

    connectedCallback() {
        this.shadowRoot.querySelector("button").addEventListener('click', this._loot.bind(this));
    }

    disconnectedCallback() {
        this.shadowRoot.querySelector(".submit").removeEventListener('click', this._loot);
    }

    _loot(e){
        e.preventDefault();

        let id = e.target.data-id;

        fetch("http://localhost:8002/loot", {
                method : "POST",
                headers: {
                    cache : "no-cache"
                },
                body : JSON.stringify({
                    id: id
                }),
            })
            .then(res => res.text()) // parse response as JSON with res.json
            .then(response => {
                alert(response)
            })
            .catch(err => {
            console.log({service:"asteroid_loot", status:"KO", error:err})
            alert("sorry, cannot loot")
        });
    }

    _render(){
        this.shadowRoot.innerHTML = `<fieldset>
            <legend>asteroid field</legend>
            <button data-id="11">loot me !</button>
            <button data-id="12">loot me !</button>
            <button data-id="13">loot me !</button>
            <br/>
            <button data-id="21">loot me !</button>
            <button data-id="22">loot me !</button>
            <button data-id="23">loot me !</button>
            <br/>
            <button data-id="31">loot me !</button>
            <button data-id="32">loot me !</button>
            <button data-id="33">loot me !</button>
        </fieldset>`
    }
}

customElements.define('hf-asteroid-loot', AsteroidLoot);