import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Activity, Users, Zap, TrendingUp, Shield, Globe } from 'lucide-react';
import './index.css';

// Placeholder Data for "Live" Feel
const INITIAL_MARKETS = [
  { id: 1, question: "BTC > $100k by Dec 1st?", yes: 65, no: 35, pool: "14,205 USDC", volume: "High" },
  { id: 2, question: "Linera Mainnet Launch Q1?", yes: 92, no: 8, pool: "8,500 USDC", volume: "Very High" },
  { id: 3, question: "Will AI Agents replace Junior Devs?", yes: 45, no: 55, pool: "5,100 USDC", volume: "Medium" },
];

const TRADERS = [
  { id: 't1', name: 'Alpha_Seeker_99', winRate: '88%', followers: 1240, profit: '+340%' },
  { id: 't2', name: 'Linera_Whale', winRate: '92%', followers: 5800, profit: '+850%' },
];

function App() {
  const [markets, setMarkets] = useState(INITIAL_MARKETS);
  const [role, setRole] = useState('viewer'); // viewer, follower, trader
  const [notifications, setNotifications] = useState([]);

  // Simulate Real-Time Updates
  useEffect(() => {
    const interval = setInterval(() => {
      setMarkets(prev => prev.map(m => ({
        ...m,
        yes: Math.min(99, Math.max(1, m.yes + (Math.random() > 0.5 ? 1 : -1))),
        no: Math.min(99, Math.max(1, m.no + (Math.random() > 0.5 ? 1 : -1))),
      })));
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  const addNotification = (msg) => {
    const id = Date.now();
    setNotifications(prev => [...prev, { id, msg }]);
    setTimeout(() => setNotifications(prev => prev.filter(n => n.id !== id)), 3000);
  };

  const handleCopyTrade = (trader) => {
    setRole('follower');
    addNotification(`Subscribed to ${trader.name}. Copy-Trading Active.`);
    
    // Simulate a trade happening 3 seconds later
    setTimeout(() => {
      addNotification(`⚡ SIGNAL RECEIVED: ${trader.name} bet YES on BTC > $100k`);
      setTimeout(() => {
        addNotification(`🚀 AUTO-EXECUTED: You bet 10 USDC on YES.`);
      }, 800);
    }, 3000);
  };

  return (
    <div className="dashboard-container">
      {/* Header */}
      <header className="flex justify-between items-center mb-10 border-b border-gray-800 pb-4">
        <div className="flex items-center gap-3">
          <Globe className="text-[var(--neon-blue)]" size={32} />
          <h1 className="text-3xl font-bold tracking-tighter">
            ECHO<span className="text-[var(--neon-blue)]">MARKETS</span>
          </h1>
        </div>
        <div className="flex gap-4 items-center">
          <div className="flex items-center gap-2 text-sm text-gray-400">
            <div className="pulse-dot"></div>
            LINERA TESTNET: CONNECTED
          </div>
          <div className="bg-gray-900 px-4 py-2 rounded border border-gray-700 font-mono">
            0x71C...9A2
          </div>
        </div>
      </header>

      <div className="grid-layout">
        {/* Main Feed: Markets */}
        <section>
          <div className="flex justify-between items-end mb-6">
            <h2 className="text-xl font-bold flex items-center gap-2">
              <Activity className="text-[var(--neon-purple)]" />
              LIVE MARKETS
            </h2>
            <span className="text-xs text-gray-500 font-mono">UPDATING EVERY 2s</span>
          </div>

          <div className="grid gap-4">
            {markets.map((m) => (
              <motion.div 
                key={m.id}
                layout
                className="market-card relative group"
              >
                <div className="flex justify-between items-start mb-4">
                  <h3 className="text-lg font-bold">{m.question}</h3>
                  <span className={`text-xs px-2 py-1 rounded ${m.volume === 'Very High' ? 'bg-red-900 text-red-200' : 'bg-gray-800'}`}>
                    VOL: {m.volume}
                  </span>
                </div>
                
                <div className="relative h-2 bg-gray-800 rounded-full overflow-hidden mb-4">
                  <motion.div 
                    className="absolute top-0 left-0 bottom-0 bg-[var(--neon-green)]"
                    animate={{ width: `${m.yes}%` }}
                    transition={{ type: "spring", stiffness: 50 }}
                  />
                </div>

                <div className="flex justify-between text-sm font-mono mb-4">
                  <span className="text-[var(--neon-green)]">YES {m.yes}%</span>
                  <span className="text-red-400">NO {100 - m.yes}%</span>
                </div>

                <div className="flex justify-between items-center mt-4 border-t border-gray-800 pt-4">
                  <div className="text-gray-400 text-xs">POOL: <span className="text-white">{m.pool}</span></div>
                  <div className="flex gap-2">
                    <button 
                      className="px-4 py-1 text-xs font-bold bg-[var(--neon-green)] text-black rounded hover:opacity-80 transition"
                      onClick={() => addNotification(`Bet Placed: YES on Market #${m.id}`)}
                    >
                      BET YES
                    </button>
                    <button 
                      className="px-4 py-1 text-xs font-bold bg-red-500 text-black rounded hover:opacity-80 transition"
                      onClick={() => addNotification(`Bet Placed: NO on Market #${m.id}`)}
                    >
                      BET NO
                    </button>
                  </div>
                </div>
              </motion.div>
            ))}
          </div>
        </section>

        {/* Sidebar: Copy Trading */}
        <aside>
          <div className="mb-8">
            <h2 className="text-xl font-bold flex items-center gap-2 mb-4">
              <Users className="text-[var(--neon-blue)]" />
              TOP TRADERS
            </h2>
            <div className="flex flex-col gap-3">
              {TRADERS.map((t) => (
                <div key={t.id} className="bg-[var(--card-bg)] border border-gray-800 p-4 rounded hover:border-[var(--neon-blue)] transition group">
                  <div className="flex justify-between items-center mb-2">
                    <span className="font-bold text-[var(--neon-blue)]">{t.name}</span>
                    <Shield size={14} className="text-yellow-500" />
                  </div>
                  <div className="grid grid-cols-2 gap-2 text-xs text-gray-400 mb-3">
                    <div>Win Rate: <span className="text-white">{t.winRate}</span></div>
                    <div>Profit: <span className="text-[var(--neon-green)]">{t.profit}</span></div>
                    <div className="col-span-2">Followers: <span className="text-white">{t.followers}</span></div>
                  </div>
                  <button 
                    className="w-full btn-cyber text-xs py-2 flex items-center justify-center gap-2"
                    onClick={() => handleCopyTrade(t)}
                  >
                    <Zap size={14} /> AUTO-COPY
                  </button>
                </div>
              ))}
            </div>
          </div>

          {/* Stats Panel */}
          <div className="stats-panel rounded">
            <h3 className="text-sm font-bold text-gray-400 mb-2">NETWORK PULSE</h3>
            <div className="text-2xl font-mono font-bold text-[var(--neon-blue)] mb-1">
              45ms
            </div>
            <div className="text-xs text-gray-500 mb-4">AVG LATENCY</div>
            
            <div className="text-2xl font-mono font-bold text-[var(--neon-purple)] mb-1">
              14,205
            </div>
            <div className="text-xs text-gray-500">ACTIVE MICROCHAINS</div>
          </div>
        </aside>
      </div>

      {/* Notifications Toast */}
      <div className="fixed bottom-8 right-8 flex flex-col gap-2 pointer-events-none">
        <AnimatePresence>
          {notifications.map((n) => (
            <motion.div
              key={n.id}
              initial={{ opacity: 0, x: 50 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: 50 }}
              className="bg-black border-l-4 border-[var(--neon-green)] text-white px-6 py-4 rounded shadow-2xl flex items-center gap-3 max-w-md"
            >
              <TrendingUp size={20} className="text-[var(--neon-green)]" />
              <span className="font-mono text-sm">{n.msg}</span>
            </motion.div>
          ))}
        </AnimatePresence>
      </div>
    </div>
  );
}

export default App;