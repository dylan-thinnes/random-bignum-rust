#!/usr/bin/perl

use List::Util qw(sum);

my $out_length = $ARGV[0] || 1000;
my $trials = $ARGV[1] || 10;

my $cell_width = 3 + (length $out_length);
my $side_width = 5 + (length $trials);

sub side_cell  { printf "%${side_width}s",   $_[0]; }
sub str_cell   { printf "%${cell_width}s",   $_[0]; }
sub int_cell   { printf "%${cell_width}d",   $_[0]; }
sub float_cell { printf "%${cell_width}.1f", $_[0]; }

@runs = ();
for my $i ( 0 .. $trials - 1 ) {
    my $out = `./target/debug/random-bignum-rust $out_length`;
    chomp $out;
    $runs[$i] = $out;
}

@results = ();
for my $i ( 0 .. $trials - 1 ) {
    my @counts = ();
    $results[$i] = \@counts;

    my $run = $runs[$i];
    for my $char ( 0 .. 9 ) {
        my @matches = $run =~ m/$char/g;
        my $count = @matches;
        $counts[$char] = $count;
    }
}

printf "%7s", "";
for my $i ( 0 .. 9 ) {
    str_cell "'$i'";
}
print "\n";

for my $i ( 0 .. $trials - 1 ) {
    printf "%7s", "RUN #$i";
    for my $c ( 0 .. 9 ) {
        int_cell $results[$i]->[$c];
    }
    int_cell sum @{ $results[$i] };
    print "\n";
}

printf "%7s", "AVG";
my $overall_avg = 0;
for my $c ( 0 .. 9 ) {
    my $char_occurences = 0;
    for my $i ( 0 .. 9 ) {
        $char_occurences += $results[$i]->[$c];
    }
    float_cell ($char_occurences / 10);
    $overall_avg += $char_occurences / 10;
}
float_cell ($overall_avg / 10);
printf "\n"
