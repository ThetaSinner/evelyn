var gulp = require('gulp');

const sass = require('gulp-sass');
const addsrc = require('gulp-add-src');
const autoprefixer = require('gulp-autoprefixer');
const pixrem = require('gulp-pixrem');
const babel = require("gulp-babel");
const sourcemaps = require("gulp-sourcemaps");
const concat = require("gulp-concat");
const cssnano = require('gulp-cssnano');
const browserify = require('browserify');
const source = require('vinyl-source-stream');
const buffer = require('vinyl-buffer');
const uglify = require('gulp-uglify');
const gutil = require('gulp-util');
const streamify = require('gulp-streamify')

const srcPaths = {
  sass: '../shared/scss/main.scss',
  css: '../shared/vendored/foundation-icon-fonts-3/foundation-icons.css',
  nodeJs: [
    '../shared/react_components/simpletask.js',
  ],
  js: [
    '../shared/angular_components/simpletask.js',
  ],
  vendoredJs: [
    '../shared/vendored/js/jquery-3.2.1-dev.js',
    '../shared/vendored/js/lodash-4.17.4-dev.js',
    '../shared/vendored/js/lodash-no-conflict.js',
    '../shared/vendored/js/angular-1.6.4-dev.js',
    '../shared/vendored/js/angular-route-1.6.4-dev.js',
    '../shared/vendored/js/underscore-1.8.3-dev.js',
    '../shared/vendored/js/backbone-1.3.3-dev.js',
    'node_modules/foundation-sites/dist/js/foundation.js',
  ],
};

const outPaths = {
  css: 'app/css',
  js: 'app/js',
};

const sassCompileSettings = {
  includePaths: [
    './node_modules/foundation-sites/scss'
  ],
  outputStyle: 'compressed', // if css compressed **file size**
};

const autoPrefixerSettings = {
  browsers: ['last 2 versions'],
  cascade: false,
};

gulp.task('sass', function() {
  return gulp.src(srcPaths.sass)
      //.pipe(sourcemaps.init())
      .pipe(sass(sassCompileSettings).on('error', sass.logError))
      .pipe(addsrc.append(srcPaths.css))
      .pipe(concat('app.css'))
      .pipe(autoprefixer(autoPrefixerSettings))
      .pipe(pixrem())
      .pipe(cssnano())
      //.pipe(sourcemaps.write())
      .pipe(gulp.dest(outPaths.css));
});

const babelSettings = {
  presets: ["babel-preset-react", "babel-preset-es2015"].map(require.resolve),
};

gulp.task('babel', function() {
  return gulp.src(srcPaths.nodeJs)
      .pipe(sourcemaps.init())
        .pipe(babel(babelSettings))
        .pipe(concat('app-node-part.js'))
      .pipe(sourcemaps.write("./"))
      .pipe(gulp.dest(outPaths.js));
});

gulp.task('browserify', ['babel'], function () {
  var b = browserify({
    entries: './app/js/app-node-part.js',
    debug: true
  });

  return b.bundle()
    .pipe(source('app-node-part.js'))
    .pipe(buffer())
    .pipe(sourcemaps.init({loadMaps: true}))
        //.pipe(uglify())
        .on('error', gutil.log)
    .pipe(sourcemaps.write('./'))
    .pipe(gulp.dest('app/js/'));
});

gulp.task('javascript', function () {
  return gulp.src(srcPaths.js)
    .pipe(concat('app.js'))
    .pipe(gulp.dest(outPaths.js));
});

gulp.task('lib', function() {
  return gulp.src(srcPaths.vendoredJs)
    .pipe(sourcemaps.init())
      .pipe(concat('lib.js'))
      //.pipe(uglify())
    .pipe(sourcemaps.write('./'))
    .pipe(gulp.dest(outPaths.js));
});

gulp.task('copy-main', function() {
  return gulp.src('src/*').pipe(gulp.dest('app/'));
});

gulp.task('copy-font-icons', function() {
  return gulp.src([
    '../shared/vendored/foundation-icon-fonts-3/foundation-icons.eot',
    '../shared/vendored/foundation-icon-fonts-3/foundation-icons.ttf',
    '../shared/vendored/foundation-icon-fonts-3/foundation-icons.woff'
  ]).pipe(gulp.dest('app/css'));
});

gulp.task('copy-font-icons-svgs', function() {
  return gulp.src('../shared/vendored/foundation-icon-fonts-3/svgs/*').pipe(gulp.dest('app/css/svgs'));
});

gulp.task('copy-partials', function() {
  return gulp.src('../shared/partials/**').pipe(gulp.dest('app/partials'));
});

gulp.task('default', ['sass', 'javascript', 'browserify', 'lib', 'copy-main', 'copy-partials', 'copy-font-icons', 'copy-font-icons-svgs'], function() {
  gulp.watch([srcPaths.sass, srcPaths.css], ['sass']);
  gulp.watch([srcPaths.js], ['javascript']);
  gulp.watch([srcPaths.nodeJs], ['browserify']);
  gulp.watch([srcPaths.vendoredJs], ['lib']);
  gulp.watch(['src/*'], ['copy-main']);
  gulp.watch(['../shared/partials/**'], ['copy-partials']);
});
