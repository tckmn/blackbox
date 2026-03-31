function renderGuess(str) {
    const el = document.createElement('div');
    el.textContent = str;
    return el;
}

function renderResponse(str) {
    return renderGuess(str);
}

function extractGuess(area) {
    const ret = area.firstChild.value;
    area.firstChild.value = '';
    return ret;
}

window.addEventListener('load', () => {
    const grid = document.getElementById('grid');
    const status = document.getElementById('status');

    const ws = new WebSocket('/ws');
    ws.addEventListener('open', () => { status.dataset.status = 'good'; });
    ws.addEventListener('error', () => { status.dataset.status = 'bad'; });
    ws.addEventListener('close', () => { status.dataset.status = 'bad'; });
    ws.addEventListener('message', msg => { const j = JSON.parse(msg.data); handlers[j.t](j.c); });

    const form = document.getElementById('guessForm');
    const area = document.getElementById('guessArea');
    form.addEventListener('submit', e => {
        e.preventDefault();
        ws.send(JSON.stringify({MakeGuess: extractGuess(area)}));
    });

    const handlers = {

        AllGuesses: guesses => {
            grid.replaceChildren(...guesses.flatMap(pair => [
                renderGuess(pair[0]),
                renderResponse(pair[1])
            ]));
        },

        OneGuess: pair => {
            grid.append(renderGuess(pair[0]));
            grid.append(renderResponse(pair[1]));
        }

    };
});
