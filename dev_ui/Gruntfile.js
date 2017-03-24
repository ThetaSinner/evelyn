module.exports = function(grunt) {
  grunt.loadNpmTasks('grunt-contrib-sass');
  grunt.loadNpmTasks('grunt-postcss');
  grunt.loadNpmTasks('grunt-contrib-copy');

  grunt.initConfig({
    sass: {
      dist: {
        options: {
          loadPath: ['node_modules/foundation-sites/scss'],
          sourcemap: 'none',
        },
        files: {
          'public/css/foundation.css': 'scss/custom-foundation.scss',
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
      dist: {
        src: 'public/css/*.css'
      }
    },
  });

  grunt.registerTask('build', ['sass:dist', 'postcss:dist']);
}
