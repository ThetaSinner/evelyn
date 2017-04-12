module.exports = function(grunt) {
  grunt.loadNpmTasks('grunt-contrib-sass');
  grunt.loadNpmTasks('grunt-postcss');
  grunt.loadNpmTasks('grunt-contrib-copy');
  grunt.loadNpmTasks('grunt-contrib-watch');

  grunt.initConfig({
    sass: {
      dev: {
        options: {
          loadPath: ['node_modules/foundation-sites/scss'],
          sourcemap: 'none',
        },
        files: {
          'app/css/app.css': '../shared/scss/main.scss',
        }
      }
    },
    postcss: {
      options: {
        processors: [
          require('pixrem')(), // add fallbacks for rem units
          require('autoprefixer')({browsers: 'last 2 versions'}), // add vendor prefixes
          require('cssnano')() // minify the result
        ]
      },
      dev: {
        src: 'app/css/*.css'
      }
    },
    copy: {
      dev: {
        files: [
          {expand: true, cwd: 'src/', src: ['**'], dest: 'app/'},
          {expand: true, cwd: '../shared/vendored/js/', src: ['**'], dest: 'app/js'},
          {expand: true, cwd: 'node_modules/foundation-sites/dist/js', src: 'foundation.js', dest: 'app/js'},
        ],
      },
    },
    watch: {
      scss: {
        files: '../shared/scss/*.scss',
        tasks: ['sass:dev'],
      },
    },
  });

  grunt.registerTask('dev-build', [
    'sass:dev',
    'postcss:dev',
    'copy:dev'
  ]);

  grunt.registerTask('build', [
    'sass:dev',
    'postcss:dev',
    'copy:dev'
  ]);
}
