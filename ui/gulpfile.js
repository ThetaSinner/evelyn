var gulp = require('gulp');

const _ = require('lodash');
const addsrc = require('gulp-add-src');
const autoprefixer = require('gulp-autoprefixer');
const babel = require("gulp-babel");
const buffer = require('vinyl-buffer');
const concat = require("gulp-concat");
const cssnano = require('gulp-cssnano');
const gutil = require('gulp-util');
const pixrem = require('gulp-pixrem');
const sass = require('gulp-sass');
const source = require('vinyl-source-stream');
const sourcemaps = require("gulp-sourcemaps");
const streamify = require('gulp-streamify');
const uglify = require('gulp-uglify');

function ResourceLocator(output_path_prefix, is_use_dev_resources) {
  this.input_path_prefix = 'src/';
  this.output_path_prefix = output_path_prefix;
  this.is_use_dev_resources = is_use_dev_resources;

  this.srcPaths = {
    index: 'index.html',
    scss_entrypoint: 'scss/main.scss',
    scss_watches: [
      'vendored/scss/_foundation_settings.scss',
      'components/**/*.scss',
    ],
    css: 'vendored/foundation-icon-fonts-3/foundation-icons.css',
    js: [
      'components/**/*.js',
      'javascript/evelyn-desktop.js',
      'javascript/controllers/*.js',
      'javascript/services/*.js',
    ],
    vendoredJs: [
      'vendored/js/jquery-3.2.1.min.js',
      'vendored/js/lodash-4.17.4.min.js',
      'vendored/js/angular-1.6.4.min.js',
      'vendored/js/angular-route-1.6.4.min.js',
      'vendored/js/underscore-1.8.3.min.js',
      'vendored/js/backbone-1.3.3.min.js',
      '../node_modules/foundation-sites/dist/js/foundation.min.js',
    ],
    foundationIconFonts: [
      'vendored/foundation-icon-fonts-3/foundation-icons.eot',
      'vendored/foundation-icon-fonts-3/foundation-icons.ttf',
      'vendored/foundation-icon-fonts-3/foundation-icons.woff'
    ],
    foundationIconSvgs: 'vendored/foundation-icon-fonts-3/svgs/*',
    htmlPartials: 'components/**/*.partial.html',
  };

  this.srcPaths.scss_watches = _.concat(this.srcPaths.scss_watches, this.srcPaths.scss_entrypoint);

  if (is_use_dev_resources) {
  	this.srcPaths.vendoredJs = _.map(this.srcPaths.vendoredJs, function (item) {
  		return _.replace(item, /(.*)\.min(\.js)/i, "$1$2");
  	});
  }

  this.outPaths = {
    css: 'css',
    js: 'js',
    index: '',
    htmlPartials: 'partials',
  };

  this.outResourceNames = {
    css: 'app.css',
    js: 'app.js',
    vendoredJs: 'lib.js',
  };
}

ResourceLocator.prototype.getSourcePaths = function(identifier) {
  _this = this;

  if (!this.srcPaths.hasOwnProperty(identifier)) {
    throw new Error('Cannot get source path for identifier [' + identifier + ']');
  }

  return _.map(_.castArray(this.srcPaths[identifier]), function (item) {
    return _this.input_path_prefix + item;
  });
};

ResourceLocator.prototype.getOutputPath = function(identifier) {
  if (!this.outPaths.hasOwnProperty(identifier)) {
    throw new Error('Cannot get output path for identifier [' + identifier + ']');
  }

  return this.output_path_prefix + this.outPaths[identifier];
};

ResourceLocator.prototype.getOutputResourceName = function(identifier) {
  if (!this.outResourceNames.hasOwnProperty(identifier)) {
    throw new Error('Cannot get output resource name for identifier [' + identifier + ']');
  }

  return this.outResourceNames[identifier];
};

ResourceLocator.prototype.isUseDevResources = function() {
  return this.is_use_dev_resources;
};

var resourceLocator = new ResourceLocator('./app/', true);

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

/*
 * This is the main styles task. It compiles all scss and concatenates the output
 * with any other css files which are specified.
 */
gulp.task('css', function() {
  var scssSources = resourceLocator.getSourcePaths('scss_entrypoint');
  var cssSoures = resourceLocator.getSourcePaths('css');

  console.log(resourceLocator.getSourcePaths('scss_watches'));

  var outputResourceName = resourceLocator.getOutputResourceName('css');
  var outputPath = resourceLocator.getOutputPath('css');

  return gulp.src(scssSources)
      //.pipe(sourcemaps.init())
      .pipe(sass(sassCompileSettings).on('error', sass.logError))
      .pipe(addsrc.append(cssSoures))
      .pipe(concat(outputResourceName))
      .pipe(autoprefixer(autoPrefixerSettings))
      .pipe(pixrem())
      .pipe(cssnano())
      //.pipe(sourcemaps.write())
      .pipe(gulp.dest(outputPath));
});

/*
 * Load all project javascript files.
 */
gulp.task('javascript', function () {
  var sources = resourceLocator.getSourcePaths('js');

  var outputResourceName = resourceLocator.getOutputResourceName('js');
  var outputPath = resourceLocator.getOutputPath('js');

  var task = gulp.src(sources)
    .pipe(sourcemaps.init())
    .pipe(concat(outputResourceName));

  if (!resourceLocator.isUseDevResources()) {
    task = task.pipe(uglify());
  }

  return task
    .pipe(sourcemaps.write('./'))
    .pipe(gulp.dest(outputPath));
});

/*
 * Load all third party javascript files.
 */
gulp.task('vendored-javascript', function() {
  var sources = resourceLocator.getSourcePaths('vendoredJs');

  var outputResourceName = resourceLocator.getOutputResourceName('vendoredJs');
  var outputPath = resourceLocator.getOutputPath('js');

  console.log(sources);

  return gulp.src(sources)
    .pipe(sourcemaps.init())
    .pipe(concat(outputResourceName))
    .pipe(sourcemaps.write('./'))
    .pipe(gulp.dest(outputPath));
});

/*
 * Copy the index html file.
 */
gulp.task('copy-index', function() {
  var source = resourceLocator.getSourcePaths('index');
  var outputPath = resourceLocator.getOutputPath('index');

  return gulp.src(source)
    .pipe(gulp.dest(outputPath));
});

/*
 * Copy html partials.
 */
gulp.task('copy-partials', function() {
  var source = resourceLocator.getSourcePaths('htmlPartials');
  var outputPath = resourceLocator.getOutputPath('htmlPartials');

  return gulp.src(source)
    .pipe(gulp.dest(outputPath));
});

/*
 * Copy foundation icon fonts.
 */
gulp.task('copy-font-icons', function() {
  var sources = resourceLocator.getSourcePaths('foundationIconFonts');
  var outputPath = resourceLocator.getOutputPath('css');

  return gulp.src(sources)
    .pipe(gulp.dest(outputPath));
});

/*
 * Copy foundation icon font svgs.
 */
gulp.task('copy-font-icons-svgs', function() {
  var sources = resourceLocator.getSourcePaths('foundationIconSvgs');
  var outputPath = resourceLocator.getOutputPath('css');

  return gulp.src(sources)
    .pipe(gulp.dest(outputPath + '/svgs'));
});

gulp.task('default', ['copy-index', 'copy-partials', 'css', 'javascript', 'vendored-javascript', 'copy-font-icons', 'copy-font-icons-svgs'], function() {
  gulp.watch(resourceLocator.getSourcePaths('index'), ['copy-index']);
  gulp.watch(resourceLocator.getSourcePaths('htmlPartials'), ['copy-partials']);
  gulp.watch(_.concat(resourceLocator.getSourcePaths('scss_watches'), resourceLocator.getSourcePaths('css')), ['css']);
  gulp.watch(resourceLocator.getSourcePaths('js'), ['javascript']);
  gulp.watch(resourceLocator.getSourcePaths('vendoredJs'), ['vendored-javascript']);
});
