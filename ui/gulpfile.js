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

function ResourceLocator(output_path_prefix, use_dev_resources) {
  this.input_path_prefix = 'src/';
  this.output_path_prefix = output_path_prefix;

  this.srcPaths = {
    scss: 'scss/main.scss',
    css: 'vendored/foundation-icon-fonts-3/foundation-icons.css',
    js: [],
    vendoredJs: [
      'vendored/js/jquery-3.2.1.js',
      'vendored/js/lodash-4.17.4.js',
      'vendored/js/angular-1.6.4.js',
      'vendored/js/angular-route-1.6.4.js',
      'vendored/js/underscore-1.8.3.js',
      'vendored/js/backbone-1.3.3.js',
      '../node_modules/foundation-sites/dist/js/foundation.js',
    ],
  };

  if (use_dev_resources) {
    this.srcPaths.vendoredJs = [
      'vendored/js/jquery-3.2.1-dev.js',
      'vendored/js/lodash-4.17.4-dev.js',
      'vendored/js/angular-1.6.4-dev.js',
      'vendored/js/angular-route-1.6.4-dev.js',
      'vendored/js/underscore-1.8.3-dev.js',
      'vendored/js/backbone-1.3.3-dev.js',
      'node_modules/foundation-sites/dist/js/foundation.js',
    ];
  }
}

ResourceLocator.prototype.getSourcePath = function(identifier) {
  // TODO identifier is a key into this.srcPaths and should return the value, or array of values
  // from srcPaths with this.input_path_prefix prepended.
};

ResourceLocator.prototype.getOutputPath = function(identifier) {
  // TODO see outPaths below and move them into this class. Prefix with
  // this.output_path_prefix
};

var resourceLocator = new ResourceLocator('app/', true);

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

/**
 * This is the main styles task. It compiles all scss and concatenates the output
 * with any other css files which are specified.
 */
// TODO rename task?
gulp.task('sass', function() {
  // TODO use resource locator
  return gulp.src(srcPaths.sass)
      //.pipe(sourcemaps.init())
      .pipe(sass(sassCompileSettings).on('error', sass.logError))

      // TODO use resource locator
      .pipe(addsrc.append(srcPaths.css))

      .pipe(concat('app.css')) // Concatenate all streamed files into app.css
      .pipe(autoprefixer(autoPrefixerSettings))
      .pipe(pixrem())
      .pipe(cssnano())
      //.pipe(sourcemaps.write())
      // TODO use resource locator
      .pipe(gulp.dest(outPaths.css));
});

/*
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
*/

/*
 * Load all project javascript files.
 */
// TODO This includes loading from the src/components folder
gulp.task('javascript', function () {
  // TODO use resource locator
  return gulp.src(srcPaths.js)
    .pipe(concat('app.js'))
    // TODO use resource locator
    .pipe(gulp.dest(outPaths.js));
});

/*
 * Load all third party javascript files.
 */
// TODO rename task
gulp.task('lib', function() {
  // TODO use resource locator
  return gulp.src(srcPaths.vendoredJs)
    .pipe(sourcemaps.init())
      .pipe(concat('lib.js'))
      //.pipe(uglify())
    .pipe(sourcemaps.write('./'))
    // TODO use resource locator
    .pipe(gulp.dest(outPaths.js));
});

// TODO this used to copy the electron files into the app to make it runnable with electron.
// This task needs to be replaced with something quite different so just ignore for now.
gulp.task('copy-main', function() {
  return gulp.src('src/*').pipe(gulp.dest('app/'));
});

gulp.task('copy-font-icons', function() {
  // TODO use resource locator
  return gulp.src([
    '../shared/vendored/foundation-icon-fonts-3/foundation-icons.eot',
    '../shared/vendored/foundation-icon-fonts-3/foundation-icons.ttf',
    '../shared/vendored/foundation-icon-fonts-3/foundation-icons.woff'
  ]).pipe(gulp.dest('app/css')); // TODO use resource locator
});

gulp.task('copy-font-icons-svgs', function() {
  // TODO use resource locator
  return gulp.src('../shared/vendored/foundation-icon-fonts-3/svgs/*').pipe(gulp.dest('app/css/svgs'));
});

// TODO Should copy partials from src/components.
// which is different from what it currently does, so probably just redo me :)
gulp.task('copy-partials', function() {
  return gulp.src('../shared/partials/**').pipe(gulp.dest('app/partials'));
});

// TODO as the above starts to be worked on this is the place to test it.
// probably remove the watches until you get it working once.
gulp.task('default', ['sass', 'javascript', 'browserify', 'lib', 'copy-main', 'copy-partials', 'copy-font-icons', 'copy-font-icons-svgs'], function() {
  // TODO use resource locator
  // TODO use resource locator
  // TODO use resource locator
  // TODO use resource locator
  gulp.watch([srcPaths.sass, srcPaths.css], ['sass']);
  gulp.watch([srcPaths.js], ['javascript']);
  gulp.watch([srcPaths.nodeJs], ['browserify']);
  gulp.watch([srcPaths.vendoredJs], ['lib']);
  gulp.watch(['src/*'], ['copy-main']);
  gulp.watch(['../shared/partials/**'], ['copy-partials']);
});
