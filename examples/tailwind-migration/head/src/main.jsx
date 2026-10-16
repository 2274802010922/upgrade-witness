import React, { useState } from 'react';
import { createRoot } from 'react-dom/client';
import './styles.css';

const notes = [
  ['01', 'Choose the flows', 'Record the interactions and visual details that matter to your users.'],
  ['02', 'Keep the context', 'Pin versions, fixtures, viewport and the exact revisions under review.'],
  ['03', 'Review the evidence', 'Inspect each observation and replay its check before accepting a change.'],
];

function App() {
  const [menuOpen, setMenuOpen] = useState(false);
  return <main className="notebook">
    <header>
      <a className="wordmark" href="/">UW / NOTEBOOK</a>
      <button data-testid="menu-toggle" className="menu-toggle" aria-expanded={menuOpen}
        aria-controls="notes-menu" onClick={() => setMenuOpen(!menuOpen)}>Menu</button>
    </header>
    {menuOpen && <nav id="notes-menu" data-testid="menu-panel" aria-label="Notebook navigation">
      <a href="#notes">Migration notes</a>
      <button className="close-menu" onClick={() => setMenuOpen(false)}>Close menu</button>
    </nav>}
    <section className="intro">
      <p className="eyebrow">ORIGINAL SYNTHETIC DEMO</p>
      <h1 data-testid="page-title">Migration notebook</h1>
      <p className="description">A small interface with explicit expectations for a dependency upgrade.</p>
      <button data-testid="primary-action" className="action-button" data-variant="outline-hidden">Review upgrade</button>
    </section>
    <section id="notes" className="columns-dashboard note-grid" data-testid="notes-grid" aria-label="Migration notes">
      {notes.map(([number, title, description]) => <article className="note" key={number}>
        <span className="note-number">{number}</span><h2>{title}</h2><p>{description}</p>
      </article>)}
    </section>
    <footer>Fixed content · Local assets · No external requests</footer>
  </main>;
}

createRoot(document.getElementById('root')).render(<App />);
