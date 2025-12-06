const { ls } = require('./index.js');
console.log('checkpoint 1');

const files = ls('.');
console.log('checkpoint 2', files);
