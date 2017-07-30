const gulp = require('gulp');
const plugins = require('gulp-load-plugins')();

const _ = require('lodash');
const minify = require('html-minifier').minify;
const source = require('vinyl-source-stream');

function ResourceLocator(output_path_prefix, is_use_dev_resources) {
    this.input_path_prefix = 'src/';
    this.output_path_prefix = output_path_prefix;
    this.is_use_dev_resources = is_use_dev_resources;

    this.srcPaths = {
        index: 'index.html',
        scss_entrypoint: 'scss/main.scss',
        scss_watches: [
            'vendored/scss/_foundation_settings.scss',
            'scss/**/*.scss',
            'components/**/*.scss',
            'vendored/foundation-building-blocks/**/*.scss',
            'vendored/font-awesome-4.7.0/scss/*.scss',
        ],
        css: [
            '../node_modules/foundation-datepicker/css/foundation-datepicker.css',
        ],
        js: [
            'components/**/*.js',
            'javascript/modules/*.js',
            'javascript/config/*.js',
            'javascript/services/*.js',
            'javascript/controllers/*.js',
            'javascript/components/*.js',
        ],
        vendoredJs: [
            'vendored/js/jquery-3.2.1.min.js',
            'vendored/js/lodash-4.17.4.min.js',
            'vendored/js/angular-1.6.4.min.js',
            'vendored/js/angular-ui-router-1.0.3.min.js',
            'vendored/js/underscore-1.8.3.min.js',
            'vendored/js/backbone-1.3.3.min.js',
            'vendored/js/moment-2.18.1.js',
            '../node_modules/foundation-sites/dist/js/foundation.min.js',
            '../node_modules/foundation-datepicker/js/foundation-datepicker.min.js',
            '../node_modules/alertify.js/dist/js/ngAlertify.js',
            'vendored/js/handlebars-v4.0.10.min.js',
        ],
        fontAwesomeFonts: [
            'vendored/font-awesome-4.7.0/FontAwesome.otf',
            'vendored/font-awesome-4.7.0/fontawesome-webfont.eot',
            'vendored/font-awesome-4.7.0/fontawesome-webfont.svg',
            'vendored/font-awesome-4.7.0/fontawesome-webfont.ttf',
            'vendored/font-awesome-4.7.0/fontawesome-webfont.woff',
            'vendored/font-awesome-4.7.0/fontawesome-webfont.woff2',
        ],
        htmlPartials: 'components/**/*.partial.html',
        electronJs: 'desktop/*.js',
    };

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
        fonts: 'fonts',
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

ResourceLocator.prototype.setOutputPathPrefix = function(path) {
    this.output_path_prefix = path;
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
    var cssSources = resourceLocator.getSourcePaths('css');

    var outputResourceName = resourceLocator.getOutputResourceName('css');
    var outputPath = resourceLocator.getOutputPath('css');

    return gulp.src(scssSources)
    //.pipe(plugins.sourcemaps.init())
    .pipe(plugins.sass(sassCompileSettings).on('error', plugins.sass.logError))
    .pipe(plugins.addSrc.append(cssSources))
    .pipe(plugins.concat(outputResourceName))
    .pipe(plugins.autoprefixer(autoPrefixerSettings))
    .pipe(plugins.pixrem())
    .pipe(plugins.cssnano())
    //.pipe(plugins.sourcemaps.write())
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
    .pipe(plugins.sourcemaps.init())
    .pipe(plugins.order([
        "components/**/*model.js",
        "components/**/*collection.js",
        "components/**/*view.js",
        "javascript/modules/*.js",
        "javascript/services/*.js",
        "javascript/controllers/*.js",
        "javascript/components/*.js",
    ], {base: resourceLocator.input_path_prefix}))
    .pipe(plugins.concat(outputResourceName))
    .pipe(plugins.fileInclude({
        basepath: '@root',
        filters: {
            cleanHtml: function (x) {
                return minify(x, {
                    collapseWhitespace: true,
                });
            }
        }
    }));

    if (!resourceLocator.isUseDevResources()) {
        task = task.pipe(plugins.uglify());
    }

    return task
    .pipe(plugins.sourcemaps.write('./'))
    .pipe(gulp.dest(outputPath));
});

/*
* Load all third party javascript files.
*/
gulp.task('vendored-javascript', function () {
    var sources = resourceLocator.getSourcePaths('vendoredJs');

    var outputResourceName = resourceLocator.getOutputResourceName('vendoredJs');
    var outputPath = resourceLocator.getOutputPath('js');

    return gulp.src(sources)
    .pipe(plugins.sourcemaps.init())
    .pipe(plugins.concat(outputResourceName))
    .pipe(plugins.sourcemaps.write('./'))
    .pipe(gulp.dest(outputPath));
});

/*
* Copy the index html file.
*/
gulp.task('copy-index', function () {
    var source = resourceLocator.getSourcePaths('index');
    var outputPath = resourceLocator.getOutputPath('index');

    return gulp.src(source)
    .pipe(gulp.dest(outputPath));
});

/*
* Copy font icons.
*/
gulp.task('copy-font-icons', function () {
    var sources = resourceLocator.getSourcePaths('fontAwesomeFonts');
    var outputPath = resourceLocator.getOutputPath('fonts');

    return gulp.src(sources)
    .pipe(gulp.dest(outputPath));
});

gulp.task('electron-js', function() {
    var sources = resourceLocator.getSourcePaths('electronJs');
    var outputPath = resourceLocator.getOutputPath('index');

    return gulp.src(sources)
        .pipe(gulp.dest(outputPath));
});

gulp.task('set-env-desktop', function() {
    resourceLocator.setOutputPathPrefix('./desktop/app/');
});

const defaultTasks = ['copy-index', 'css', 'javascript', 'vendored-javascript', 'copy-font-icons'];

function defaultWatches() {
    gulp.watch(resourceLocator.getSourcePaths('index'), ['copy-index']);
    gulp.watch(
        _.concat(
            resourceLocator.getSourcePaths('scss_watches'),
            resourceLocator.getSourcePaths('css')
        ), ['css']);
    gulp.watch(
        _.concat(
            resourceLocator.getSourcePaths('js'),
            resourceLocator.getSourcePaths('htmlPartials')
        ), ['javascript']);
    gulp.watch(resourceLocator.getSourcePaths('vendoredJs'), ['vendored-javascript']);
}

gulp.task('default', defaultTasks, function () {
    defaultWatches();
});

gulp.task('desktop', _.concat(['set-env-desktop', 'electron-js'], defaultTasks), function () {
    defaultWatches();

    // TODO add watches for electron.
});
