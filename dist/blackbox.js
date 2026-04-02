let itype, otype, extractGuess;

function wrap(s) {
    const el = document.createElement('div');
    el.textContent = s;
    return el;
}

function take(el) {
    const ret = el.value;
    el.value = '';
    return ret;
}

const render = {
    STR: s => wrap(JSON.parse(s)),
    NUM: s => wrap(s)
};

const makeGuessField = {

    STR: area => {
        const el = document.createElement('input');
        area.append(el);
        return () => JSON.stringify(take(el));
    },

    NUM: area => {
        const el = document.createElement('input');
        el.setAttribute('type', 'number');
        el.setAttribute('step', 'any');
        area.append(el);
        return () => take(el);
    }

};

window.addEventListener('load', () => {
    const grid = document.getElementById('grid');
    const status = document.getElementById('status');

    let ws, retry;
    const connect = () => {
        delete status.dataset.status;
        ws = new WebSocket('/ws');
        ws.addEventListener('open', () => { status.dataset.status = 'good'; retry = undefined; });
        ws.addEventListener('error', reconnect);
        ws.addEventListener('close', reconnect);
        ws.addEventListener('message', msg => { const j = JSON.parse(msg.data); handlers[j.t](j.c); });
    };
    const reconnect = () => {
        status.dataset.status = 'bad';
        ws.close();
        setTimeout(connect, retry ? (retry*=1.5) : (retry=1000));
    };
    connect();

    const form = document.getElementById('guessForm');
    const area = document.getElementById('guessArea');
    form.addEventListener('submit', e => {
        e.preventDefault();
        ws.send(JSON.stringify({MakeGuess: extractGuess()}));
    });

    const handlers = {

        SetPuzzle: puzzle => {
            itype = puzzle[1];
            otype = puzzle[2];
            area.replaceChildren();
            extractGuess = makeGuessField[itype](area);
            grid.replaceChildren();
        },

        AllGuesses: guesses => {
            grid.replaceChildren(...guesses.flatMap(pair => [
                render[itype](pair[0]),
                render[otype](pair[1])
            ]));
        },

        OneGuess: pair => {
            grid.append(render[itype](pair[0]));
            grid.append(render[otype](pair[1]));
        }

    };

    document.getElementById('collapse').addEventListener('click', () => { document.body.classList.add('collapsed'); });
    document.getElementById('uncollapse').addEventListener('click', () => { document.body.classList.remove('collapsed'); });
    document.getElementById('col1').addEventListener('click', () => { document.body.classList.remove('col2'); document.body.classList.add('col1'); });
    document.getElementById('col2').addEventListener('click', () => { document.body.classList.remove('col1'); document.body.classList.add('col2'); });
});
