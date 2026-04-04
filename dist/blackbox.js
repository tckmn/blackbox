let itype, otype, extractGuess, history;

function wrap(s) {
    const el = document.createElement('div');
    el.textContent = s;
    return el;
}

function take(el) {
    const ret = el.value.trim();
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
    const gridcont = document.getElementsByTagName('main')[0];
    const grid = document.getElementById('grid');
    const status = document.getElementById('status');
    const puzname = document.getElementById('puzname');

    let ws, retry, reconnecting = false;
    const connect = () => {
        reconnecting = false;
        delete status.dataset.status;
        ws = new WebSocket('/ws');
        ws.addEventListener('open', () => { status.dataset.status = 'good'; retry = undefined; });
        ws.addEventListener('error', reconnect);
        ws.addEventListener('close', reconnect);
        ws.addEventListener('message', msg => { const j = JSON.parse(msg.data); handlers[j.t](j.c); });
    };
    const reconnect = () => {
        if (!reconnecting) {
            reconnecting = true;
            status.dataset.status = 'bad';
            ws.close();
            setTimeout(connect, retry ? (retry*=1.5) : (retry=1000));
        }
    };
    connect();

    const form = document.getElementById('guessForm');
    const area = document.getElementById('guessArea');
    form.addEventListener('submit', e => {
        e.preventDefault();
        const guess = extractGuess();
        if (history[guess]) {
            history[guess].scrollIntoView();
            // TODO this is temporary
            history[guess].animate([
                { backgroundColor: 'rgba(255,200,50,0)' },
                { backgroundColor: 'rgba(255,200,50,255)' },
                { backgroundColor: 'rgba(255,200,50,0)' },
                { backgroundColor: 'rgba(255,200,50,255)' },
                { backgroundColor: 'rgba(255,200,50,0)' }
            ], 1000);
        } else ws.send(JSON.stringify({MakeGuess: guess}));
    });

    const handlers = {

        SetPuzzle: puzzle => {
            puzname.textContent = puzzle[0];
            itype = puzzle[1];
            otype = puzzle[2];
            area.replaceChildren();
            extractGuess = makeGuessField[itype](area);
            history = {};
            grid.replaceChildren();
        },

        AllGuesses: guesses => {
            grid.replaceChildren(...guesses.flatMap(pair => [
                (history[pair[0]] = render[itype](pair[0])),
                render[otype](pair[1])
            ]));
        },

        OneGuess: pair => {
            const atBottom = gridcont.scrollHeight - gridcont.scrollTop - gridcont.clientHeight < 2;
            grid.append(history[pair[0]] = render[itype](pair[0]));
            grid.append(render[otype](pair[1]));
            if (atBottom) grid.scrollIntoView(false);
        }

    };

    document.getElementById('collapse').addEventListener('click', () => { document.body.classList.add('collapsed'); });
    document.getElementById('uncollapse').addEventListener('click', () => { document.body.classList.remove('collapsed'); });
    document.getElementById('col1').addEventListener('click', () => { document.body.classList.remove('col2'); document.body.classList.add('col1'); });
    document.getElementById('col2').addEventListener('click', () => { document.body.classList.remove('col1'); document.body.classList.add('col2'); });
});
