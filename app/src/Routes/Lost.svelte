<div>
    <p>Looks like you got lost. Lets fix that.</p>

    <a href="/#/">
        <button> <span>Bring me back</span> </button>
    </a>
</div>

<!-- button shit copy and pasted from w3 https://www.w3schools.com/css/tryit.asp?filename=trycss_buttons_animate1 -->
<style>
    div {
        background-color: white;
        border: 2px solid black;
        padding: 30px;
        width: fit-content;
        max-width: 40em;
        font-size: 20px;
    }
    button {
        display: inline-block;
        border-radius: 4px;
        background-color: #3a3a3a;
        border: none;
        color: #ffffff;
        text-align: center;
        font-size: 28px;
        padding: 20px;
        width: fit-content;
        transition: all 0.5s;
        cursor: pointer;
        margin: 5px;
    }
    button span {
        cursor: pointer;
        display: inline-block;
        position: relative;
        transition: 0.5s;
    }

    button span:after {
        content: ">>";
        position: absolute;
        opacity: 0;
        top: 0;
        right: -20px;
        transition: 0.5s;
    }

    button:hover span {
        padding-right: 50px;
    }

    button:hover span:after {
        opacity: 1;
        right: 0;
    }
</style>
