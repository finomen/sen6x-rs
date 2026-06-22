'use strict'

module.exports = require('conventional-changelog-conventionalcommits')({
  types: [
    { type: 'feat',     section: 'Features' },
    { type: 'fix',      section: 'Bug Fixes' },
    { type: 'docs',     section: 'Documentation' },
    { type: 'doc',      section: 'Documentation' },
    { type: 'perf',     section: 'Performance' },
    { type: 'refactor', section: 'Refactoring' },
    { type: 'test',     section: 'Testing' },
    { type: 'build',    section: 'Build' },
    { type: 'ci',       hidden: true },
    { type: 'chore',    hidden: true },
    { type: 'style',    hidden: true }
  ]
})
