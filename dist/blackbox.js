function renderGuess(str) {
    const el = document.createElement('div');
    el.textContent = str;
    return el;
}

function renderResponse(str) {
    return renderGuess(str);
}

function extractGuess(area) {
    return area.firstChild.value;
}

window.addEventListener('load', () => {
    const grid = document.getElementById('grid');

    const ws = new WebSocket('/ws');
    ws.addEventListener('open', () => { console.log('open'); });
    ws.addEventListener('error', () => { console.log('error'); });
    ws.addEventListener('message', msg => { const j = JSON.parse(msg.data); handlers[j.t](j.c); });

    const form = document.getElementById('guessForm');
    const area = document.getElementById('guessArea');
    form.addEventListener('submit', e => {
        e.preventDefault();
        ws.send(JSON.stringify({MakeGuess: extractGuess(area)}));
    });

    const handlers = {

        OneGuess: pair => {
            grid.append(renderGuess(pair[0]));
            grid.append(renderResponse(pair[0]));
        }

    };
});
