#!/usr/bin/env ruby --encoding utf-8

require "docopt"
require "webloc"

doc = <<DOCOPT
Webloc file format utility.

Usage:
  #{__FILE__} read <filename>
  #{__FILE__} save <filename> [<url>]
  #{__FILE__} -h | --help
  #{__FILE__} --version

Options:
  -h, --help  Print this message and exit.
  --version   Show version info and exit.

DOCOPT

if __FILE__ == $0
    begin
        args = Docopt::docopt(doc, version: "1.0.0")
        if args['read']
            puts Webloc.load(args['<filename>']).url
        elsif args['save']
            url = args['<url>']
            if url.nil?
                url = STDIN.gets.strip
            end
            Webloc.new(url).save(args['<filename>'])
        end
    rescue Docopt::Exit => e
        puts e.message
    end
end
