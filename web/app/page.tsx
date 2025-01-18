'use client';

import { useState } from 'react';
import { Github, Coffee } from 'lucide-react';

export default function Home() {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText('cargo install ur-commit-mentor');
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen flex flex-col bg-white dark:bg-gray-900 text-gray-900 dark:text-white">
     
      <div className="absolute inset-0 overflow-hidden pointer-events-none opacity-30 sm:opacity-60 min-h-screen">
        <div className="absolute -top-1/2 -left-1/2 w-[200%] sm:w-full h-full bg-gradient-to-br from-gray-100 to-transparent dark:from-gray-800 dark:to-transparent rounded-full blur-3xl" />
        <div className="absolute -bottom-1/2 -right-1/2 w-[200%] sm:w-full h-full bg-gradient-to-tl from-gray-100 to-transparent dark:from-gray-800 dark:to-transparent rounded-full blur-3xl" />
        <div className="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:14px_14px] sm:bg-[size:24px_24px] dark:bg-[linear-gradient(to_right,#ffffff12_1px,transparent_1px),linear-gradient(to_bottom,#ffffff12_1px,transparent_1px)]" />
      </div>

      {/* Top Navigation */}
      <nav className="absolute w-full z-10 border-b border-gray-200/50 dark:border-gray-800/50">
        <div className="max-w-7xl mx-auto px-3 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-14 sm:h-16">
            <p className="text-sm sm:text-base text-center">
            Made with{" "}
            <span className="text-red-500 animate-pulse">❤️</span> by{" "}
            <a 
              href="https://github.com/ddoemonn"
              target="_blank"
              rel="noopener noreferrer"
              className="font-medium hover:text-gray-900 dark:hover:text-white transition-colors hover:underline"
            >
              @ddoemonn
            </a>
          </p>
            
            <div className="flex items-center gap-2 sm:gap-4">
              <a
                href="https://github.com/ddoemonn/ur-commit-mentor"
                target="_blank"
                rel="noopener noreferrer"
                 className="flex items-center gap-2 px-4 py-2 bg-black text-white rounded-lg hover:bg-black/90 transition-colors"
              >
                <Github className="w-5 h-5" />
                <span className="hidden sm:inline font-medium">Repository</span>
              </a>
              <a
                href="https://www.buymeacoffee.com/ozergklp"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-2 px-4 py-2 bg-[#FFDD00] text-black rounded-lg hover:bg-[#FFDD00]/90 transition-colors"
                title="Buy me a coffee"
              >
                <Coffee className="w-5 h-5" />
                <span className="hidden sm:inline font-medium">Buy me a coffee</span>
              </a>
            </div>
          </div>
        </div>
      </nav>

      <div className="relative flex-1 max-w-7xl mx-auto px-3 sm:px-6 lg:px-8 pt-24 sm:pt-36">
        <main className="flex flex-col items-center justify-center text-center">
          {/* Hero Section */}
          <div className="space-y-4 sm:space-y-6 mb-12 sm:mb-16">
            <h1 className="text-4xl sm:text-5xl md:text-7xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-gray-900 via-gray-700 to-gray-900 dark:from-white dark:via-gray-300 dark:to-white pb-2">
              UR Commit Mentor
            </h1>
            <p className="text-base sm:text-lg md:text-2xl max-w-3xl mx-auto mt-4 sm:mt-6 text-gray-600 dark:text-gray-300">
              AI-powered code review insights for better commit messages using Claude AI
            </p>
          </div>

          {/* Installation Section */}
          <div className="w-full max-w-3xl mb-12 sm:mb-20">
            <div className="bg-gray-50/50 backdrop-blur-sm dark:bg-gray-800/50 rounded-2xl p-6 shadow-lg border border-gray-200/50 dark:border-gray-700/50">
              <p className="text-sm mb-4 text-gray-600 dark:text-gray-300">Install using Cargo:</p>
              <div className="relative group">
                <pre className="bg-gray-900 dark:bg-black rounded-xl p-4 overflow-x-auto transition-transform group-hover:scale-[1.01]">
                  <code className="text-white font-mono">cargo install ur-commit-mentor</code>
                </pre>
                <button 
                  onClick={handleCopy}
                  className={`absolute right-4 top-1/2 -translate-y-1/2 p-2 rounded-lg 
                    ${copied 
                      ? 'bg-green-500 text-white' 
                      : 'text-gray-400 hover:text-white hover:bg-gray-700/50'
                    } transition-all duration-300`}
                >
                  {copied ? "Copied!" : "Copy"}
                </button>
              </div>
            </div>
          </div>

          {/* Features */}
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-8 w-full pb-12 sm:pb-0">
            {[
              {
                title: "Pre-Push Analysis",
                description: "Review your commits before pushing to catch potential issues"
              },
              {
                title: "AI-Powered Insights",
                description: "Get intelligent suggestions about code quality and best practices"
              },
              {
                title: "Quick Feedback",
                description: "Understand the impact of your changes without waiting for human review"
              }
            ].map((feature, index) => (
              <div 
                key={index}
                className="group p-8 border border-gray-200/50 dark:border-gray-700/50 rounded-2xl 
                  hover:border-gray-300 dark:hover:border-gray-600 transition-all duration-300 
                  hover:shadow-lg bg-white/50 dark:bg-gray-800/50 backdrop-blur-sm
                  hover:-translate-y-1"
              >
                <h3 className="text-xl font-semibold mb-3 group-hover:text-gray-900 dark:group-hover:text-white transition-colors">
                  {feature.title}
                </h3>
                <p className="text-gray-600 dark:text-gray-300">
                  {feature.description}
                </p>
              </div>
            ))}
          </div>
        </main>
      </div>
    </div>
  );
}
